use std::sync::Arc;

use dioxus::server::{DioxusRouterExt, ServeConfig};
use futures_util::{SinkExt, StreamExt};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{app::App, models::ChatMessage, server::state::AppState};

pub fn api_routes(state: Arc<AppState>) -> Router {
    // todo
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(false);

    let ws_router = Router::new()
        .route("/api/ws", get(ws_handler))
        .with_state(state);

    let dioxus_config = ServeConfig::new();

    Router::new()
        .merge(ws_router) // API роутер
        .serve_dioxus_application(dioxus_config, App) // Frontend
        .layer(cors)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // sender (отправка) и receiver (прием)
    let (mut ws_sender, mut ws_receiver) = socket.split();

    let mut rx = state.tx.subscribe();

    // Прием сообщений от клиента
    let tx_clone = state.tx.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(parsed) = serde_json::from_str::<ChatMessage>(&text) {
                        let _ = tx_clone.send(parsed);
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Отправка сообщений из broadcast клиенту
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Ждём завершения любой из задач
    tokio::select! {
        _ = recv_task => {},
        _ = send_task => {},
    }
}
