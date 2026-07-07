use dioxus::prelude::*;

use crate::api;
use crate::state::{use_auth, use_toaster};
use crate::Route;

const INPUT: &str = "w-full rounded-lg border border-slate-200 bg-slate-50 px-3.5 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 outline-none transition focus:border-indigo-400 focus:bg-white focus:ring-2 focus:ring-indigo-100";
const LABEL: &str = "block text-sm font-medium text-slate-700 mb-1.5";
const SUBMIT: &str = "w-full rounded-lg bg-indigo-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-indigo-700 disabled:opacity-60 disabled:cursor-not-allowed";

#[component]
fn AuthShell(title: String, subtitle: String, children: Element) -> Element {
    rsx! {
        div { class: "min-h-[calc(100vh-4rem)] grid place-items-center px-4 py-12 bg-gradient-to-b from-slate-50 to-indigo-50/40",
            div { class: "w-full max-w-sm",
                div { class: "flex flex-col items-center mb-6 text-center",
                    span { class: "grid place-items-center w-12 h-12 rounded-2xl bg-indigo-600 text-white text-2xl mb-3", "📖" }
                    h1 { class: "text-xl font-semibold text-slate-900", "{title}" }
                    p { class: "text-sm text-slate-500 mt-1", "{subtitle}" }
                }
                div { class: "rounded-2xl border border-slate-200 bg-white p-6 shadow-sm shadow-slate-900/5",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    let mut auth = use_auth();
    let mut toaster = use_toaster();
    let nav = use_navigator();

    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut show_pwd = use_signal(|| false);
    let mut loading = use_signal(|| false);

    rsx! {
        AuthShell { title: "欢迎回来", subtitle: "登录以借阅与管理图书",
            form {
                class: "flex flex-col gap-4",
                onsubmit: move |evt| {
                    evt.prevent_default();
                    if loading() { return; }
                    let username_v = username().trim().to_string();
                    let password_v = password();
                    if username_v.is_empty() || password_v.is_empty() {
                        toaster.error("请填写用户名和密码");
                        return;
                    }
                    loading.set(true);
                    spawn(async move {
                        match api::login(&username_v, &password_v).await {
                            Ok(token) => match api::me(&token).await {
                                Ok(user) => {
                                    auth.set_session(token, user);
                                    toaster.success("登录成功");
                                    nav.push(Route::Books {});
                                }
                                Err(err) => toaster.error(err),
                            },
                            Err(err) => toaster.error(err),
                        }
                        loading.set(false);
                    });
                },
                div {
                    label { class: LABEL, "用户名" }
                    input {
                        class: INPUT,
                        placeholder: "请输入用户名",
                        autocomplete: "username",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                    }
                }
                div {
                    label { class: LABEL, "密码" }
                    div { class: "relative",
                        input {
                            class: "{INPUT} pr-16",
                            r#type: if show_pwd() { "text" } else { "password" },
                            autocomplete: "current-password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                        button {
                            class: "absolute right-1 top-1/2 -translate-y-1/2 px-2 py-1 text-xs text-slate-400 hover:text-slate-600 rounded transition",
                            r#type: "button",
                            onclick: move |_| show_pwd.set(!show_pwd()),
                            if show_pwd() { "隐藏" } else { "显示" }
                        }
                    }
                }
                button { class: SUBMIT, r#type: "submit", disabled: loading(),
                    if loading() { "登录中…" } else { "登录" }
                }
            }
            p { class: "mt-5 text-center text-sm text-slate-500",
                "还没有账号？"
                Link { to: Route::Register {}, class: "font-medium text-indigo-600 hover:text-indigo-700", " 立即注册" }
            }
        }
    }
}

#[component]
pub fn Register() -> Element {
    let mut auth = use_auth();
    let mut toaster = use_toaster();
    let nav = use_navigator();

    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut show_pwd = use_signal(|| false);
    let mut show_cfm = use_signal(|| false);
    let mut loading = use_signal(|| false);

    rsx! {
        AuthShell { title: "创建账号", subtitle: "加入云图书馆，开启阅读之旅",
            form {
                class: "flex flex-col gap-4",
                onsubmit: move |evt| {
                    evt.prevent_default();
                    if loading() { return; }
                    let username_v = username().trim().to_string();
                    let password_v = password();
                    if username_v.is_empty() || password_v.is_empty() {
                        toaster.error("请完整填写所有字段");
                        return;
                    }
                    if username_v.len() < 2 || username_v.len() > 20 {
                        toaster.error("用户名长度需 2-20 位");
                        return;
                    }
                    if password_v.len() < 6 {
                        toaster.error("密码长度至少 6 位");
                        return;
                    }
                    if password_v != confirm() {
                        toaster.error("两次输入的密码不一致");
                        return;
                    }
                    loading.set(true);
                    spawn(async move {
                        match api::register(&username_v, &password_v).await {
                            Ok(_) => match api::login(&username_v, &password_v).await {
                                Ok(token) => match api::me(&token).await {
                                    Ok(user) => {
                                        auth.set_session(token, user);
                                        toaster.success("注册成功，已自动登录");
                                        nav.push(Route::Books {});
                                    }
                                    Err(err) => toaster.error(err),
                                },
                                Err(_) => {
                                    toaster.success("注册成功，请登录");
                                    nav.push(Route::Login {});
                                }
                            },
                            Err(err) => toaster.error(err),
                        }
                        loading.set(false);
                    });
                },
                div {
                    label { class: LABEL, "用户名" }
                    input {
                        class: INPUT,
                        placeholder: "请输入用户名",
                        autocomplete: "username",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                    }
                }
                div {
                    label { class: LABEL, "密码" }
                    div { class: "relative",
                        input {
                            class: "{INPUT} pr-16",
                            r#type: if show_pwd() { "text" } else { "password" },
                            autocomplete: "new-password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                        button {
                            class: "absolute right-1 top-1/2 -translate-y-1/2 px-2 py-1 text-xs text-slate-400 hover:text-slate-600 rounded transition",
                            r#type: "button",
                            onclick: move |_| show_pwd.set(!show_pwd()),
                            if show_pwd() { "隐藏" } else { "显示" }
                        }
                    }
                }
                div {
                    label { class: LABEL, "确认密码" }
                    div { class: "relative",
                        input {
                            class: "{INPUT} pr-16",
                            r#type: if show_cfm() { "text" } else { "password" },
                            autocomplete: "new-password",
                            value: "{confirm}",
                            oninput: move |e| confirm.set(e.value()),
                        }
                        button {
                            class: "absolute right-1 top-1/2 -translate-y-1/2 px-2 py-1 text-xs text-slate-400 hover:text-slate-600 rounded transition",
                            r#type: "button",
                            onclick: move |_| show_cfm.set(!show_cfm()),
                            if show_cfm() { "隐藏" } else { "显示" }
                        }
                    }
                }
                button { class: SUBMIT, r#type: "submit", disabled: loading(),
                    if loading() { "提交中…" } else { "注册" }
                }
            }
            p { class: "mt-5 text-center text-sm text-slate-500",
                "已有账号？"
                Link { to: Route::Login {}, class: "font-medium text-indigo-600 hover:text-indigo-700", " 去登录" }
            }
        }
    }
}
