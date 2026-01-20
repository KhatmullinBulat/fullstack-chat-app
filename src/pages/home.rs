use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::{
    icons::bs_icons::{BsPersonCircle, BsSend},
    Icon,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_storage::{LocalStorage, Storage};

use crate::{
    components::{self, button::Button, textarea::Textarea},
    models::ChatMessage,
    router::Route,
};

#[derive(Debug)]
enum ChatAction {
    Connect,
    SetUser(String),
    SendMessage(String),
}

struct ChatState {
    messages: Signal<Vec<ChatMessage>>,
    err_msg: Signal<Option<String>>,
    tx: UnboundedSender<ChatAction>,
}

fn use_chat_service() -> ChatState {
    let messages = use_signal(|| Vec::new());
    let err_msg = use_signal(|| None);

    let task = use_coroutine(move |mut rx: UnboundedReceiver<ChatAction>| {
        let mut messages = messages;
        let mut err_msg = err_msg;

        async move {
            let mut ws_write: Option<SplitSink<WebSocket, Message>> = None;

            let mut internal_username: Option<String> = None;

            while let Some(action) = rx.next().await {
                match action {
                    ChatAction::SetUser(name) => {
                        internal_username = Some(name.clone());
                    }
                    ChatAction::Connect => {
                        let ws_url = "ws://localhost:8080/api/ws";

                        match WebSocket::open(ws_url) {
                            Ok(ws) => {
                                let (write, mut read) = ws.split();

                                ws_write = Some(write);

                                spawn(async move {
                                    while let Some(msg) = read.next().await {
                                        if let Ok(Message::Text(text)) = msg {
                                            if let Ok(parsed) =
                                                serde_json::from_str::<ChatMessage>(&text)
                                            {
                                                messages.write().push(parsed);
                                            }
                                        }
                                    }
                                });
                            }
                            Err(e) => err_msg.set(Some(e.to_string())),
                        }
                    }
                    ChatAction::SendMessage(text) => {
                        if let (Some(user), Some(writer)) = (&internal_username, &mut ws_write) {
                            let msg_obj = ChatMessage {
                                user: user.clone(),
                                text,
                            };

                            if let Ok(json) = serde_json::to_string(&msg_obj) {
                                let _ = writer.send(Message::Text(json)).await;
                            }
                        } else {
                            tracing::warn!("Не могу отправить: нет юзера или соединения");
                        }
                    }
                }
            }
        }
    });

    ChatState {
        messages,
        err_msg,
        tx: task.tx(),
    }
}

/// Home page
#[component]
pub fn Home() -> Element {
    let chat = use_chat_service();
    let navigator = use_navigator();

    let mut current_user = use_signal(|| String::new());
    let mut current_message = use_signal(|| String::new());

    let tx_auth = chat.tx.clone();

    use_effect(move || {
        if let Ok(stored_name) = LocalStorage::get::<String>("username") {
            if !stored_name.is_empty() {
                current_user.set(stored_name.clone());

                let _ = tx_auth.unbounded_send(ChatAction::SetUser(stored_name));
                let _ = tx_auth.unbounded_send(ChatAction::Connect);
            } else {
                navigator.replace(Route::Login {});
            }
        } else {
            navigator.replace(Route::Login {});
        }
    });

    let tx_send = chat.tx.clone();

    let mut send_handler = move || {
        let text = current_message();
        if !text.trim().is_empty() {
            let _ = tx_send.unbounded_send(ChatAction::SendMessage(text));
            current_message.set(String::new());
        }
    };

    let mut send_handler_on_enter = send_handler.clone();

    rsx! {
        div { class: "h-screen flex flex-col gap-3 max-w-[800px] mx-auto py-10 px-5",

            if let Some(err) = chat.err_msg.read().as_ref() {
                div { class: "text-red-500", "{err}" }
            }

            button {
                class: "flex items-center gap-3 w-max cursor-pointer",
                onclick: move |_| {
                    navigator.push(Route::Login {});
                },
                Icon { icon: BsPersonCircle, height: 35, width: 35 }
                p { "{current_user}" }
            }

            div { class: "flex-1 overflow-y-auto border rounded-lg bg-gray-50",
                div { class: "flex flex-col justify-end min-h-full gap-2 p-4",
                    for msg in chat.messages.read().iter() {
                        MessageCard {
                            user: msg.user.clone(),
                            text: msg.text.clone(),
                            current_user: current_user(),
                        }
                    }
                }
            }

            form {
                class: "flex justify-end",
                onsubmit: move |e: FormEvent| {
                    e.prevent_default();
                    send_handler();
                },
                Textarea {
                    class: "textarea",
                    variant: components::textarea::TextareaVariant::Outline,
                    placeholder: "Пиши",
                    value: current_message,
                    maxlength: 300,
                    oninput: move |e: FormEvent| {
                        current_message.set(e.value());
                    },
                    onkeydown: move |e: KeyboardEvent| {
                        // Если нажат Enter и не нажат Shift
                        if e.key() == Key::Enter && !e.modifiers().contains(Modifiers::SHIFT) {
                            e.prevent_default();
                            send_handler_on_enter();
                        }
                    },
                }

                Button { class: "button", r#type: "submit",
                    Icon { icon: BsSend }
                }
            }
        }
    }
}

#[component]
fn MessageCard(user: String, text: String, current_user: String) -> Element {
    let is_my_message = user == current_user;

    let base = "max-w-[75%] rounded-2xl px-4 py-3 text-sm leading-relaxed break-words shadow-lg";

    let class = if is_my_message {
        format!(
            "{} self-end bg-gradient-to-br from-blue-500 to-blue-600 text-white",
            base
        )
    } else {
        format!("{} self-start bg-gray-200 text-gray-900", base)
    };

    rsx! {
        div { class: "{class}", "{user}: {text}" }
    }
}
