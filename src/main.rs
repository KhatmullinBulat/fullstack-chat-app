mod app;
mod components;
mod models;
mod pages;
mod router;
mod server;

#[cfg(feature = "web")]
fn main() {
    use crate::app::App;

    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::Router;
    use tokio::net::TcpListener;

    use crate::server::{routes::api_routes, state::AppState};

    let state = AppState::new();

    let port_str = std::env::var("PORT").unwrap_or("8080".to_string());
    let port = port_str.parse::<u16>().unwrap();

    let ip_str = std::env::var("IP").unwrap_or("0.0.0.0".to_string());
    let ip = ip_str.parse::<std::net::IpAddr>().unwrap();

    let addr = SocketAddr::from((ip, port));

    let api_router = api_routes(state);

    let app_router = Router::new().merge(api_router);

    println!("Server is running at http://{addr}");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}
