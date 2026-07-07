use dioxus::prelude::*;

use crate::state::{use_auth, use_toaster, Toast, ToastKind};
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let mut auth = use_auth();
    let nav = use_navigator();
    let mut toaster = use_toaster();

    let link = "px-3 py-2 rounded-lg text-sm font-medium text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors";
    let active = "px-3 py-2 rounded-lg text-sm font-medium text-indigo-600 bg-indigo-50";

    rsx! {
        header { class: "sticky top-0 z-40 bg-white/85 backdrop-blur border-b border-slate-200",
            nav { class: "mx-auto max-w-6xl px-4 h-16 flex items-center gap-2",
                Link {
                    to: Route::Books {},
                    class: "flex items-center gap-2 mr-4 shrink-0",
                    span { class: "grid place-items-center w-9 h-9 rounded-xl bg-indigo-600 text-white text-lg", "📖" }
                    span { class: "text-lg font-semibold tracking-tight text-slate-900", "云图书馆" }
                }

                Link { to: Route::Books {}, class: link, active_class: active, "浏览图书" }
                if auth.is_authed() {
                    Link { to: Route::Borrowed {}, class: link, active_class: active, "我的借阅" }
                    Link { to: Route::Profile {}, class: link, active_class: active, "账号" }
                }
                if auth.is_admin() {
                    Link { to: Route::Admin {}, class: link, active_class: active, "管理" }
                }

                div { class: "flex-1" }

                match auth.user_value() {
                    Some(user) => rsx! {
                        div { class: "flex items-center gap-3",
                            div { class: "hidden sm:flex flex-col items-end leading-tight",
                                span { class: "text-sm font-medium text-slate-800", "{user.username}" }
                            }
                            if user.is_admin() {
                                span { class: "px-2 py-0.5 rounded-full text-xs font-semibold bg-amber-100 text-amber-700", "管理员" }
                            }
                            button {
                                class: "px-3 py-1.5 rounded-lg text-sm font-medium text-slate-600 border border-slate-200 hover:bg-slate-50 transition-colors",
                                onclick: move |_| {
                                    auth.logout();
                                    nav.push(Route::Books {});
                                    toaster.success("已退出登录");
                                },
                                "退出"
                            }
                        }
                    },
                    None => rsx! {
                        div { class: "flex items-center gap-2",
                            Link { to: Route::Login {}, class: link, "登录" }
                            Link {
                                to: Route::Register {},
                                class: "px-3.5 py-2 rounded-lg text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 transition-colors",
                                "注册"
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn ToastHost() -> Element {
    let toaster = use_toaster();
    let items = toaster.items();

    rsx! {
        div { class: "fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80 max-w-[calc(100vw-2rem)]",
            for toast in items() {
                ToastCard { key: "{toast.id}", toast: toast.clone() }
            }
        }
    }
}

#[component]
fn ToastCard(toast: Toast) -> Element {
    let mut toaster = use_toaster();
    let id = toast.id;

    use_effect(move || {
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(3400).await;
            toaster.dismiss(id);
        });
    });

    let (accent, icon) = match toast.kind {
        ToastKind::Success => ("border-l-emerald-500", "✓"),
        ToastKind::Error => ("border-l-rose-500", "!"),
    };
    let badge = match toast.kind {
        ToastKind::Success => "bg-emerald-100 text-emerald-600",
        ToastKind::Error => "bg-rose-100 text-rose-600",
    };

    rsx! {
        div { class: "flex items-start gap-3 rounded-xl border border-slate-200 border-l-4 {accent} bg-white shadow-lg shadow-slate-900/5 px-4 py-3 animate-[fadeIn_0.2s_ease-out]",
            span { class: "grid place-items-center w-6 h-6 shrink-0 rounded-full text-sm font-bold {badge}", "{icon}" }
            p { class: "flex-1 text-sm text-slate-700 leading-snug break-words", "{toast.message}" }
            button {
                class: "text-slate-300 hover:text-slate-500 transition-colors leading-none text-lg",
                onclick: move |_| toaster.dismiss(id),
                "×"
            }
        }
    }
}

#[component]
pub fn Spinner(label: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-3 py-16 text-slate-400",
            div { class: "w-8 h-8 rounded-full border-2 border-slate-200 border-t-indigo-500 animate-spin" }
            span { class: "text-sm", "{label}" }
        }
    }
}

#[component]
pub fn EmptyState(title: String, hint: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-2 py-20 text-center",
            span { class: "text-4xl grayscale opacity-60", "🔍" }
            p { class: "text-base font-medium text-slate-600", "{title}" }
            p { class: "text-sm text-slate-400", "{hint}" }
        }
    }
}
