use crate::pages::Home;
use crate::pages::Login;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/:..route")]
    NotFound { route: Vec<String>},
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "Страница не найдена" }
            p { "Путь: {route:?}" }
            Link { to: Route::Home {}, "На главную" }
        }
    }
}
