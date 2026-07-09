use dioxus::prelude::*;

use crate::api;
use crate::models::BorrowedBook;
use crate::state::{use_auth, use_toaster};
use crate::ui::EmptyState;
use crate::Route;

#[component]
pub fn Borrowed() -> Element {
    let auth = use_auth();
    let _toaster = use_toaster();

    use_effect(move || {
        if let Some(token) = auth.token_value() {
            let mut auth = auth;
            spawn(async move {
                if let Ok(user) = api::me(&token).await {
                    auth.set_user(user);
                }
            });
        }
    });

    if !auth.is_authed() {
        return rsx! {
            div { class: "mx-auto max-w-md px-4 py-24 text-center",
                span { class: "text-4xl", "🔒" }
                h1 { class: "mt-4 text-xl font-semibold text-slate-900", "请先登录" }
                p { class: "mt-1 text-sm text-slate-500", "登录后即可查看你的借阅记录。" }
                Link {
                    to: Route::Login {},
                    class: "mt-5 inline-block rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700",
                    "去登录"
                }
            }
        };
    }

    let Some(user) = auth.user_value() else {
        return rsx! { crate::ui::Spinner { label: "加载中…" } };
    };

    let borrowed = user.borrowed_books.clone();
    let initial = user
        .username
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    rsx! {
        div { class: "mx-auto max-w-4xl px-4 py-8",
            div { class: "flex items-center gap-4 rounded-2xl border border-slate-200 bg-white p-6 shadow-sm shadow-slate-900/5",
                div { class: "grid place-items-center w-16 h-16 shrink-0 rounded-2xl bg-gradient-to-br from-indigo-500 to-violet-500 text-white text-2xl font-bold",
                    "{initial}"
                }
                div { class: "min-w-0",
                    h1 { class: "text-xl font-semibold text-slate-900", "{user.username}" }
                }
                div { class: "ml-auto text-right",
                    div { class: "text-2xl font-semibold text-indigo-600", "{borrowed.len()}" }
                    div { class: "text-xs text-slate-400", "本借阅中" }
                }
            }

            h2 { class: "text-lg font-semibold text-slate-900 mt-8 mb-4", "借阅记录" }

            if borrowed.is_empty() {
                EmptyState { title: "还没有借阅任何图书", hint: "去图书馆藏页面挑选一本喜欢的书吧。" }
            } else {
                div { class: "flex flex-col gap-3",
                    for book in borrowed {
                        BorrowedItem { key: "{book.id}", book: book.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn BorrowedItem(book: BorrowedBook) -> Element {
    let auth = use_auth();
    let mut toaster = use_toaster();
    let mut busy = use_signal(|| false);
    let user_id = auth.user_value().map(|u| u.id);

    let do_return = move |_| {
        if busy() {
            return;
        }
        let Some(ref token) = auth.token_value() else {
            return;
        };
        let Some(uid) = user_id else {
            return;
        };
        busy.set(true);
        let token = token.clone();
        spawn(async move {
            match api::return_book(&token, uid, book.id).await {
                Ok(msg) => {
                    toaster.success(msg);
                    if let Ok(user) = api::me(&token).await {
                        let mut auth = auth;
                        auth.set_user(user);
                    }
                }
                Err(e) => toaster.error(e),
            }
            busy.set(false);
        });
    };

    rsx! {
        div { class: "flex items-center gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm shadow-slate-900/5",
            div { class: "grid place-items-center w-10 h-14 shrink-0 rounded-md bg-slate-100 text-slate-400 text-lg", "📕" }
            div { class: "min-w-0 flex-1",
                h3 { class: "font-medium text-slate-900 truncate", "{book.name}" }
                p { class: "text-sm text-slate-500 truncate", "{book.author}" }
            }
            span { class: "rounded-md bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-500", "#{book.id}" }
            button {
                class: "rounded-lg border border-slate-200 px-3.5 py-1.5 text-sm font-medium text-slate-700 transition hover:bg-slate-50 disabled:opacity-50",
                disabled: busy(),
                onclick: do_return,
                "归还"
            }
        }
    }
}
