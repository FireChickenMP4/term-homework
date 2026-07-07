use dioxus::prelude::*;

use crate::api;
use crate::models::UserBrief;
use crate::state::{use_auth, use_toaster};
use crate::ui::Spinner;
use crate::Route;

const TAB: &str = "px-4 py-2 text-sm font-medium rounded-lg transition";
const TAB_ACTIVE: &str = "bg-indigo-100 text-indigo-700";
const TAB_INACTIVE: &str = "text-slate-500 hover:text-slate-700 hover:bg-slate-100";

#[component]
pub fn Admin() -> Element {
    let auth = use_auth();
    let _nav = use_navigator();

    if !auth.is_authed() {
        return rsx! {
            div { class: "mx-auto max-w-md px-4 py-24 text-center",
                span { class: "text-4xl", "🔒" }
                h1 { class: "mt-4 text-xl font-semibold text-slate-900", "请先登录" }
                p { class: "mt-1 text-sm text-slate-500", "登录后即可访问管理面板。" }
                Link { to: Route::Login {}, class: "mt-5 inline-block rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700", "去登录" }
            }
        };
    }
    let Some(user) = auth.user_value() else {
        return rsx! { Spinner { label: "加载中…" } };
    };
    if !user.is_admin() {
        return rsx! {
            div { class: "mx-auto max-w-md px-4 py-24 text-center",
                span { class: "text-4xl", "🔒" }
                h1 { class: "mt-4 text-xl font-semibold text-slate-900", "需要管理员权限" }
                p { class: "mt-1 text-sm text-slate-500", "只有管理员才能访问此页面。" }
                Link { to: Route::Books {}, class: "mt-5 inline-block rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700", "返回首页" }
            }
        };
    }

    let mut tab = use_signal(|| 0);

    let tab0_cls = if tab() == 0 {
        format!("{TAB} {TAB_ACTIVE}")
    } else {
        format!("{TAB} {TAB_INACTIVE}")
    };
    let tab1_cls = if tab() == 1 {
        format!("{TAB} {TAB_ACTIVE}")
    } else {
        format!("{TAB} {TAB_INACTIVE}")
    };

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            h1 { class: "text-2xl font-semibold tracking-tight text-slate-900 mb-6", "管理面板" }

            div { class: "flex items-center gap-2 mb-6",
                button { class: "{tab0_cls}", onclick: move |_| tab.set(0), "用户管理" }
                button { class: "{tab1_cls}", onclick: move |_| tab.set(1), "借阅记录" }
            }

            if tab() == 0 {
                UserManage {}
            } else {
                BorrowedList {}
            }
        }
    }
}

#[component]
fn UserManage() -> Element {
    let auth = use_auth();
    let _toaster = use_toaster();

    let mut users = use_resource(move || async move {
        let Some(ref token) = auth.token_value() else {
            return Err("未登录".into());
        };
        api::list_users(token).await
    });

    rsx! {
        div { class: "rounded-xl border border-slate-200 bg-white shadow-sm shadow-slate-900/5 overflow-hidden",
            table { class: "w-full text-sm",
                thead {
                    tr { class: "bg-slate-50 text-left text-xs font-medium text-slate-500 uppercase tracking-wider",
                        th { class: "px-4 py-3", "ID" }
                        th { class: "px-4 py-3", "用户名" }
                        th { class: "px-4 py-3", "角色" }
                        th { class: "px-4 py-3 text-right", "操作" }
                    }
                }
                tbody {
                    match users() {
                        None => rsx! { tr { td { class: "px-4 py-8 text-center text-slate-400", colspan: "4", "加载中…" } } },
                        Some(Err(e)) => rsx! { tr { td { class: "px-4 py-8 text-center text-rose-500", colspan: "4", "{e}" } } },
                        Some(Ok(list)) => {
                            let list = list.clone();
                            let n = auth.user_value().map(|u| u.id);
                            rsx! {
                                for u in list {
                                    UserRow { key: "{u.id}", user: u.clone(), current_user_id: n, on_changed: move |_| users.restart() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UserRow(user: UserBrief, current_user_id: Option<i32>, on_changed: EventHandler<()>) -> Element {
    let auth = use_auth();
    let mut toaster = use_toaster();
    let mut busy = use_signal(|| false);

    let is_self = current_user_id == Some(user.id);
    let is_admin = user.permission == "admin";

    let check_demoted = {
        let auth = auth;
        move |err: &str| {
            if err == "需要管理员权限" {
                let mut auth = auth;
                spawn(async move {
                    if let Some(t) = auth.token_value() {
                        if let Ok(u) = api::me(&t).await {
                            auth.set_user(u);
                        }
                    }
                });
            }
        }
    };

    let do_toggle = move |_| {
        if busy() {
            return;
        }
        let Some(ref token) = auth.token_value() else {
            return;
        };
        busy.set(true);
        let token = token.clone();
        let new_perm = if is_admin { "user" } else { "admin" };
        let ck = check_demoted.clone();
        spawn(async move {
            match api::set_permission(&token, user.id, new_perm).await {
                Ok(msg) => {
                    toaster.success(msg);
                    on_changed.call(());
                }
                Err(e) => {
                    ck(&e);
                    toaster.error(e);
                }
            }
            busy.set(false);
        });
    };

    let do_delete = move |_| {
        if busy() {
            return;
        }
        let Some(ref token) = auth.token_value() else {
            return;
        };
        busy.set(true);
        let token = token.clone();
        let ck = check_demoted.clone();
        spawn(async move {
            match api::admin_delete_user(&token, user.id).await {
                Ok(msg) => {
                    toaster.success(msg);
                    on_changed.call(());
                }
                Err(e) => {
                    ck(&e);
                    toaster.error(e);
                }
            }
            busy.set(false);
        });
    };

    rsx! {
        tr { class: "border-t border-slate-100 hover:bg-slate-50/50",
            td { class: "px-4 py-3 text-slate-500", "{user.id}" }
            td { class: "px-4 py-3 font-medium text-slate-900",
                "{user.username}"
                if is_self {
                    span { class: "ml-2 text-xs text-slate-400", "（自己）" }
                }
            }
            td { class: "px-4 py-3",
                if is_admin {
                    span { class: "inline-block rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-700", "管理员" }
                } else {
                    span { class: "inline-block rounded-full bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-500", "用户" }
                }
            }
            td { class: "px-4 py-3 text-right",
                div { class: "flex items-center justify-end gap-2",
                    button {
                        class: "rounded-lg border border-slate-200 px-3 py-1.5 text-xs font-medium text-slate-600 transition hover:bg-slate-50 disabled:opacity-50",
                        disabled: busy() || is_self,
                        onclick: do_toggle,
                        if is_admin { "降为用户" } else { "升为管理员" }
                    }
                    button {
                        class: "rounded-lg border border-rose-200 px-3 py-1.5 text-xs font-medium text-rose-600 transition hover:bg-rose-50 disabled:opacity-50",
                        disabled: busy() || is_self,
                        onclick: do_delete,
                        "删除"
                    }
                }
            }
        }
    }
}

#[component]
fn BorrowedList() -> Element {
    let auth = use_auth();

    let records = use_resource(move || async move {
        let Some(ref token) = auth.token_value() else {
            return Err("未登录".into());
        };
        api::list_borrowed(token).await
    });

    rsx! {
        div { class: "rounded-xl border border-slate-200 bg-white shadow-sm shadow-slate-900/5 overflow-hidden",
            table { class: "w-full text-sm",
                thead {
                    tr { class: "bg-slate-50 text-left text-xs font-medium text-slate-500 uppercase tracking-wider",
                        th { class: "px-4 py-3", "用户" }
                        th { class: "px-4 py-3", "图书" }
                        th { class: "px-4 py-3", "图书 ID" }
                    }
                }
                tbody {
                    match records() {
                        None => rsx! { tr { td { class: "px-4 py-8 text-center text-slate-400", colspan: "3", "加载中…" } } },
                        Some(Err(e)) => rsx! { tr { td { class: "px-4 py-8 text-center text-rose-500", colspan: "3", "{e}" } } },
                        Some(Ok(list)) => {
                            if list.is_empty() {
                                rsx! { tr { td { class: "px-4 py-8 text-center text-slate-400", colspan: "3", "暂无借阅记录" } } }
                            } else {
                                let list = list.clone();
                                rsx! {
                                    for r in list {
                                        tr { class: "border-t border-slate-100 hover:bg-slate-50/50",
                                            td { class: "px-4 py-3 font-medium text-slate-900", "{r.username}" }
                                            td { class: "px-4 py-3 text-slate-700", "{r.book_name}" }
                                            td { class: "px-4 py-3 text-slate-500", "#{r.book_id}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
