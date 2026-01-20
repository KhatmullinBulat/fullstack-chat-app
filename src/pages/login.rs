use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsKey, BsPerson},
    Icon,
};
use gloo_storage::{LocalStorage, Storage};

use crate::router::Route;

// Login page
#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();

    let mut username = use_signal(|| String::new());
    let save_username = move || {
        if username().trim().is_empty() {
            let _ = LocalStorage::set("username", "Гость");
        } else {
            let _ = LocalStorage::set("username", username().trim().to_string());
        }

        navigator.push(Route::Home {});
    };

    rsx! {
        div { class: "h-screen flex flex-col items-center justify-center bg-gradient p-4",

            div { class: "max-w-xl w-full rounded-3xl rounded-b-none card-shadow bg-white p-8",
                h1 { class: "text-2xl lg:text-4xl font-bold text-gray-800 mb-3",
                    "С возвращением!"
                }

                p { class: "text-gray-600",
                    "Войдите в свой аккаунт, чтобы продолжить общение"
                }
            }

            div { class: "card-shadow max-w-xl w-full rounded-3xl rounded-t-none p-8 text-white",
                h2 { class: "text-2xl lg:text-3xl font-bold mb-2", "Вход в аккаунт" }
                p { class: "text-blue-100 mb-8",
                    "Пожалуйста, введите свои учетные данные"
                }

                form {
                    class: "flex flex-col gap-8",
                    onsubmit: move |e: FormEvent| {
                        e.prevent_default();

                        save_username();
                    },

                    div { class: "flex flex-col gap-2",
                        label { r#for: "name", class: "text-sm font-medium",
                            div { class: "flex items-center gap-2.5",
                                Icon { icon: BsPerson }
                                p { "Имя" }
                            }
                        }
                        input {
                            id: "name",
                            name: "name",
                            required: true,
                            class: "w-full backdrop-blur-sm bg-white/10 border border-white/20 rounded-xl text-white placeholder-blue-200 input-focus mb-2 pl-3 pr-3 py-3",
                            placeholder: "Ваше имя",
                            oninput: move |e: FormEvent| {
                                username.set(e.value());
                            },
                        }
                        p { class: "text-blue-200 text-xs",
                            "Введите ваше имя или никнейм"
                        }
                    }

                    div { class: "flex flex-col gap-2",
                        label { r#for: "password", class: "text-sm font-medium",
                            div { class: "flex items-center gap-2.5",
                                Icon { icon: BsKey }
                                p { "Пароль" }
                            }
                        }
                        input {
                            id: "password",
                            name: "password",
                            required: false,
                            class: "w-full backdrop-blur-sm bg-white/10 border border-white/20 rounded-xl text-white placeholder-blue-200 input-focus mb-2 pl-3 pr-3 py-3",
                            placeholder: "Пароль",
                                                // oninput: move |e: FormEvent| {
                        //     password.set(e.value());
                        // },
                        }
                        p { class: "text-blue-200 text-xs",
                            "Внимание пароль в данный момент необязателен"
                        }
                    }

                    button {
                        r#type: "submit",
                        class: "w-full cursor-pointer bg-white text-blue-600 font-semibold py-3 px-4 rounded-xl btn-transition hover:bg-blue-50 focus:ring-4 focus:ring-white/30",
                        "Войти"
                    }
                }
            }
        }
    }
}
