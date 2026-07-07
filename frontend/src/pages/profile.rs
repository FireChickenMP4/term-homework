use dioxus::prelude::*;

use crate::api;
use crate::state::{use_auth, use_toaster};
use crate::Route;

const INPUT: &str = "w-full rounded-lg border border-slate-200 bg-slate-50 px-3.5 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 outline-none transition focus:border-indigo-400 focus:bg-white focus:ring-2 focus:ring-indigo-100";
const LABEL: &str = "block text-sm font-medium text-slate-700 mb-1.5";

#[component]
pub fn Profile() -> Element {
    let auth = use_auth();

    if !auth.is_authed() {
        return rsx! {
            div { class: "mx-auto max-w-md px-4 py-24 text-center",
                span { class: "text-4xl", "🔒" }
                h1 { class: "mt-4 text-xl font-semibold text-slate-900", "请先登录" }
                p { class: "mt-1 text-sm text-slate-500", "登录后即可管理账号。" }
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

    rsx! {
        div { class: "mx-auto max-w-lg px-4 py-8",
            div { class: "rounded-2xl border border-slate-200 bg-white p-6 shadow-sm shadow-slate-900/5",
                h1 { class: "text-xl font-semibold text-slate-900 mb-1", "账号设置" }
                p { class: "text-sm text-slate-500 mb-6", "修改用户名或密码。" }
                ProfileSettings { user_id: user.id }
            }
        }
    }
}

#[component]
fn ProfileSettings(user_id: i32) -> Element {
    let mut auth = use_auth();
    let mut toaster = use_toaster();

    let mut username = use_signal(String::new);
    let mut old_password = use_signal(String::new);
    let mut new_password = use_signal(String::new);
    let mut show_old = use_signal(|| false);
    let mut show_new = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let mut confirm_delete = use_signal(|| false);

    rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |evt| {
                evt.prevent_default();
                if busy() { return; }
                let Some(ref token) = auth.token_value() else { return; };
                let name_v = username().trim().to_string();
                let old_v = old_password();
                let new_v = new_password();
                if name_v.is_empty() || old_v.is_empty() {
                    toaster.error("请填写用户名和当前密码");
                    return;
                }
                if name_v.len() < 2 || name_v.len() > 20 {
                    toaster.error("用户名长度需 2-20 位");
                    return;
                }
                if !new_v.is_empty() && new_v.len() < 6 {
                    toaster.error("新密码长度至少 6 位");
                    return;
                }
                busy.set(true);
                let token = token.clone();
                spawn(async move {
                    match api::update_profile(&token, user_id, &old_v, &name_v, &new_v).await {
                        Ok(msg) => {
                            toaster.success(msg);
                            if let Ok(user) = api::me(&token).await {
                                let mut auth = auth;
                                auth.set_user(user);
                            }
                            old_password.set(String::new());
                            new_password.set(String::new());
                        }
                        Err(e) => toaster.error(e),
                    }
                    busy.set(false);
                });
            },
            div {
                label { class: LABEL, "新用户名" }
                input {
                    class: INPUT,
                    placeholder: "修改用户名",
                    value: "{username}",
                    oninput: move |e| username.set(e.value()),
                }
            }
            div {
                label { class: LABEL, "当前密码" }
                div { class: "relative",
                    input {
                        class: "{INPUT} pr-14",
                        r#type: if show_old() { "text" } else { "password" },
                        value: "{old_password}",
                        oninput: move |e| old_password.set(e.value()),
                    }
                    button {
                        class: "absolute right-1 top-1/2 -translate-y-1/2 px-2 py-1 text-xs text-slate-400 hover:text-slate-600 rounded transition",
                        r#type: "button",
                        onclick: move |_| show_old.set(!show_old()),
                        if show_old() { "隐藏" } else { "显示" }
                    }
                }
            }
            div {
                label { class: LABEL, "新密码（选填）" }
                div { class: "relative",
                    input {
                        class: "{INPUT} pr-14",
                        r#type: if show_new() { "text" } else { "password" },
                        value: "{new_password}",
                        oninput: move |e| new_password.set(e.value()),
                    }
                    button {
                        class: "absolute right-1 top-1/2 -translate-y-1/2 px-2 py-1 text-xs text-slate-400 hover:text-slate-600 rounded transition",
                        r#type: "button",
                        onclick: move |_| show_new.set(!show_new()),
                        if show_new() { "隐藏" } else { "显示" }
                    }
                }
            }
            button {
                class: "self-start rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700 disabled:opacity-60",
                r#type: "submit",
                disabled: busy(),
                if busy() { "保存中…" } else { "保存修改" }
            }
        }

        div { class: "border-t border-slate-200 pt-6 mt-6",
            h3 { class: "text-sm font-medium text-rose-600 mb-3", "危险操作" }
            p { class: "text-xs text-slate-500 mb-3", "注销后账号和数据将永久删除，不可恢复。" }
            button {
                class: "rounded-lg border border-rose-300 px-4 py-2 text-sm font-medium text-rose-600 transition hover:bg-rose-50 disabled:opacity-50",
                disabled: busy(),
                onclick: move |_| confirm_delete.set(true),
                "注销账号"
            }
        }

        if confirm_delete() {
            div {
                class: "fixed inset-0 z-50 grid place-items-center bg-slate-900/40 backdrop-blur-sm px-4",
                onclick: move |_| confirm_delete.set(false),
                div {
                    class: "w-full max-w-sm rounded-2xl border border-slate-200 bg-white p-6 shadow-xl text-center",
                    onclick: move |e| e.stop_propagation(),
                    span { class: "text-4xl", "⚠️" }
                    h3 { class: "mt-3 text-lg font-semibold text-slate-900", "确认注销账号？" }
                    p { class: "mt-1 text-sm text-slate-500", "此操作不可撤销，账号和所有数据将永久删除。" }
                    div { class: "flex justify-center gap-3 mt-6",
                        button {
                            class: "rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition hover:bg-slate-50",
                            onclick: move |_| confirm_delete.set(false),
                            "取消"
                        }
                        button {
                            class: "rounded-lg bg-rose-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-rose-700 disabled:opacity-60",
                            disabled: busy(),
                            onclick: move |_| {
                                if busy() { return; }
                                let Some(ref token) = auth.token_value() else { return; };
                                confirm_delete.set(false);
                                busy.set(true);
                                let token = token.clone();
                                spawn(async move {
                                    match api::self_delete(&token, user_id).await {
                                        Ok(msg) => {
                                            toaster.success(msg);
                                            auth.logout();
                                        }
                                        Err(e) => toaster.error(e),
                                    }
                                    busy.set(false);
                                });
                            },
                            if busy() { "处理中…" } else { "确认注销" }
                        }
                    }
                }
            }
        }
    }
}
