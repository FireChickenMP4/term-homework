use dioxus::prelude::*;

mod api;
mod models;
mod pages;
mod state;
mod ui;

use pages::{Admin, Books, Borrowed, Login, Profile, Register};
use ui::{Navbar, ToastHost};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Books {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/profile")]
    Profile {},
    #[route("/borrowed")]
    Borrowed {},
    #[route("/admin")]
    Admin {},
    #[route("/:..path")]
    NotFound { path: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide global state and restore any saved session.
    let auth = state::provide_state();

    use_effect(move || {
        let mut auth = auth;
        if let Some(token) = auth.token_value() {
            spawn(async move {
                let token = match api::refresh(&token).await {
                    Ok(new_token) => {
                        // 保存续期后的 token，前端不感知
                        new_token
                    }
                    Err(_) => {
                        // refresh 也失败（token 彻底过期），清除登录态
                        auth.logout();
                        return;
                    }
                };
                match api::me(&token).await {
                    Ok(user) => auth.set_session(token, user),
                    Err(_) => auth.logout(),
                }
            });
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn NotFound(path: Vec<String>) -> Element {
    rsx! {
        div { class: "mx-auto max-w-md px-4 py-24 text-center",
            span { class: "text-5xl", "🔍" }
            h1 { class: "mt-4 text-xl font-semibold text-slate-900", "页面不存在" }
            p { class: "mt-1 text-sm text-slate-500", "路径 /{path.join(\"/\")} 未找到。" }
            Link { to: Route::Books {}, class: "mt-5 inline-block rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700", "返回首页" }
        }
    }
}

/// App layout: navigation bar, page outlet and toast overlay.
#[component]
fn Shell() -> Element {
    rsx! {
        div { class: "min-h-screen bg-slate-50 text-slate-900",
            Navbar {}
            main { Outlet::<Route> {} }
            ToastHost {}
        }
    }
}
