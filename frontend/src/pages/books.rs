use dioxus::prelude::*;

use crate::api;
use crate::models::Book;
use crate::state::{use_auth, use_book_cache, use_toaster};
use crate::ui::{EmptyState, Spinner};
use crate::Route;

const INPUT: &str = "w-full rounded-lg border border-slate-200 bg-white px-3.5 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 outline-none transition focus:border-indigo-400 focus:ring-2 focus:ring-indigo-100";

#[derive(Clone, PartialEq)]
struct Query {
    keyword: String,
    page: i32,
}

#[component]
pub fn Books() -> Element {
    let auth = use_auth();
    let mut toaster = use_toaster();
    let mut cache = use_book_cache();

    let mut keyword_input = use_signal(String::new);
    let mut query = use_signal(|| Query {
        keyword: String::new(),
        page: 1,
    });
    let mut refresh = use_signal(|| 0u32);

    let mut editor_open = use_signal(|| false);
    let mut editor_id = use_signal(|| Option::<i32>::None);
    let mut editor_name = use_signal(String::new);
    let mut editor_author = use_signal(String::new);
    let mut editor_description = use_signal(String::new);
    let mut editor_total = use_signal(|| 1i32);
    let mut editor_busy = use_signal(|| false);

    let mut books = use_resource(move || {
        let q = query();
        let _ = refresh();
        async move { api::search_books(&q.keyword, q.page).await }
    });

    use_effect(move || {
        if let Some(Ok(resp)) = books() {
            cache.insert_many(&resp.items);
        }
    });

    let reload_user = move || {
        if let Some(token) = auth.token_value() {
            let mut auth = auth;
            spawn(async move {
                if let Ok(user) = api::me(&token).await {
                    auth.set_user(user);
                }
            });
        }
    };

    let page = query().page;
    let result = books();
    let total = match &result {
        Some(Ok(resp)) => resp.total,
        _ => 0,
    };
    let count = match &result {
        Some(Ok(resp)) => resp.items.len(),
        _ => 0,
    };
    let has_prev = page > 1;
    let total_pages = if total > 0 { ((total - 1) / 9) + 1 } else { 1 };
    let has_next = page < total_pages && count > 0;

    let is_authed = auth.is_authed();
    let hint = if is_authed {
        "搜索书名、作者或描述关键词即可借阅。"
    } else {
        "搜索书名、作者或描述关键词，登录后即可借阅。"
    };

    let open_create = move |_| {
        editor_id.set(None);
        editor_name.set(String::new());
        editor_author.set(String::new());
        editor_description.set(String::new());
        editor_total.set(1);
        editor_open.set(true);
    };

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            div { class: "flex flex-wrap items-end justify-between gap-4 mb-6",
                div {
                    h1 { class: "text-2xl font-semibold tracking-tight text-slate-900", "图书馆藏" }
                    p { class: "text-sm text-slate-500 mt-1", "{hint}" }
                }
                if auth.is_admin() {
                    button {
                        class: "inline-flex items-center gap-1.5 rounded-lg bg-slate-900 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-slate-800",
                        onclick: open_create,
                        span { class: "text-base leading-none", "+" }
                        "新增图书"
                    }
                }
            }

            div { class: "rounded-xl border border-slate-200 bg-white p-4 shadow-sm shadow-slate-900/5 mb-6",
                form {
                    class: "flex flex-col sm:flex-row gap-3",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        query.set(Query {
                            keyword: keyword_input().trim().to_string(),
                            page: 1,
                        });
                    },
                    input {
                        class: INPUT,
                        placeholder: "搜索书名、作者或描述关键词",
                        value: "{keyword_input}",
                        oninput: move |e| keyword_input.set(e.value()),
                    }
                    div { class: "flex gap-2 shrink-0",
                        button {
                            class: "rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-indigo-700",
                            r#type: "submit",
                            "搜索"
                        }
                        button {
                            class: "rounded-lg border border-slate-200 px-4 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-slate-50",
                            r#type: "button",
                            onclick: move |_| {
                                keyword_input.set(String::new());
                                query.set(Query { keyword: String::new(), page: 1 });
                            },
                            "清空"
                        }
                    }
                }
            }

            if total > 0 {
                div { class: "mb-4 text-xs text-slate-400",
                    "共 {total} 本图书"
                }
            }

            match result {
                None => rsx! { Spinner { label: "正在加载图书…" } },
                Some(Err(err)) => rsx! {
                    div { class: "rounded-xl border border-rose-200 bg-rose-50 p-6 text-center",
                        p { class: "text-sm font-medium text-rose-700", "加载失败：{err}" }
                        button {
                            class: "mt-3 rounded-lg border border-rose-300 px-4 py-2 text-sm font-medium text-rose-700 transition hover:bg-rose-100",
                            onclick: move |_| books.restart(),
                            "重试"
                        }
                    }
                },
                Some(Ok(resp)) if resp.items.is_empty() => rsx! {
                    EmptyState { title: "没有找到相关图书", hint: "换个关键词或清空筛选条件再试试。" }
                },
                Some(Ok(resp)) => rsx! {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4",
                        for book in resp.items {
                            BookCard {
                                key: "{book.id}",
                                book: book.clone(),
                                borrowed: auth.user_value().is_some_and(|u| u.has_borrowed(book.id)),
                                on_changed: move |_| {
                                    reload_user();
                                    refresh.set(refresh() + 1);
                                },
                                on_edit: move |b: Book| {
                                    editor_id.set(Some(b.id));
                                    editor_name.set(b.name.clone());
                                    editor_author.set(b.author.clone());
                                    editor_description.set(b.description.clone());
                                    editor_total.set(b.total);
                                    editor_open.set(true);
                                },
                                on_deleted: move |_| refresh.set(refresh() + 1),
                            }
                        }
                    }

                    div { class: "flex items-center justify-center gap-4 mt-8",
                        button {
                            class: "rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed",
                            disabled: !has_prev,
                            onclick: move |_| {
                                let mut q = query();
                                q.page -= 1;
                                query.set(q);
                            },
                            "上一页"
                        }
                        span { class: "text-sm text-slate-500", "第 {page} / {total_pages} 页" }
                        button {
                            class: "rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed",
                            disabled: !has_next,
                            onclick: move |_| {
                                let mut q = query();
                                q.page += 1;
                                query.set(q);
                            },
                            "下一页"
                        }
                    }
                },
            }
        }

        if editor_open() {
            div {
                class: "fixed inset-0 z-50 grid place-items-center bg-slate-900/40 backdrop-blur-sm px-4",
                onclick: move |_| editor_open.set(false),
                div {
                    class: "w-full max-w-md rounded-2xl border border-slate-200 bg-white p-6 shadow-xl",
                    onclick: move |e| e.stop_propagation(),
                    h2 { class: "text-lg font-semibold text-slate-900 mb-4",
                        if editor_id().is_some() { "编辑图书" } else { "新增图书" }
                    }
                    form {
                        class: "flex flex-col gap-4",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            if editor_busy() { return; }
                            let Some(token) = auth.token_value() else {
                                toaster.error("请先登录");
                                return;
                            };
                            let name_v = editor_name().trim().to_string();
                            let author_v = editor_author().trim().to_string();
                            let desc_v = editor_description().trim().to_string();
                            let total_v = editor_total();
                            if name_v.is_empty() || author_v.is_empty() {
                                toaster.error("书名和作者不能为空");
                                return;
                            }
                            if total_v < 1 {
                                toaster.error("数量至少为 1");
                                return;
                            }
                            let id = editor_id();
                            editor_busy.set(true);
                            spawn(async move {
                                let outcome = match id {
                                    Some(id) => api::update_book(&token, id, &name_v, &author_v, &desc_v, total_v).await,
                                    None => api::create_book(&token, &name_v, &author_v, &desc_v, total_v).await,
                                };
                                match outcome {
                                    Ok(msg) => {
                                        toaster.success(msg);
                                        editor_open.set(false);
                                        refresh.set(refresh() + 1);
                                    }
                                    Err(err) => toaster.error(err),
                                }
                                editor_busy.set(false);
                            });
                        },
                        div {
                            label { class: "block text-sm font-medium text-slate-700 mb-1.5", "书名" }
                            input {
                                class: INPUT,
                                placeholder: "请输入书名",
                                value: "{editor_name}",
                                oninput: move |e| editor_name.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-slate-700 mb-1.5", "作者" }
                            input {
                                class: INPUT,
                                placeholder: "请输入作者",
                                value: "{editor_author}",
                                oninput: move |e| editor_author.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-slate-700 mb-1.5", "描述" }
                            textarea {
                                class: "{INPUT}",
                                style: "resize: vertical; field-sizing: content; max-height: 240px;",
                                placeholder: "图书简介（可选）",
                                value: "{editor_description}",
                                oninput: move |e| editor_description.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-slate-700 mb-1.5", "总数量" }
                            input {
                                class: INPUT,
                                r#type: "number",
                                min: "1",
                                value: "{editor_total}",
                                oninput: move |e| {
                                    if let Ok(n) = e.value().parse::<i32>() {
                                        editor_total.set(n);
                                    }
                                },
                            }
                        }
                        div { class: "flex justify-end gap-2 pt-2",
                            button {
                                class: "rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition hover:bg-slate-50",
                                r#type: "button",
                                onclick: move |_| editor_open.set(false),
                                "取消"
                            }
                            button {
                                class: "rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-indigo-700 disabled:opacity-60",
                                r#type: "submit",
                                disabled: editor_busy(),
                                if editor_busy() { "保存中…" } else { "保存" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BookCard(
    book: Book,
    borrowed: bool,
    on_changed: EventHandler<()>,
    on_edit: EventHandler<Book>,
    on_deleted: EventHandler<()>,
) -> Element {
    let auth = use_auth();
    let mut toaster = use_toaster();
    let nav = use_navigator();
    let mut busy = use_signal(|| false);
    let user_id = auth.user_value().map(|u| u.id);

    let id = book.id;
    let edit_book = book.clone();

    let palette = [
        "from-indigo-500 to-violet-500",
        "from-sky-500 to-cyan-500",
        "from-emerald-500 to-teal-500",
        "from-amber-500 to-orange-500",
        "from-rose-500 to-pink-500",
    ];
    let grad = palette[(id.unsigned_abs() as usize) % palette.len()];
    let initial = book
        .name
        .chars()
        .find(|c| !c.is_whitespace())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    let available_pct = if book.total > 0 {
        book.available as f64 / book.total as f64
    } else {
        0.0
    };

    let do_borrow = move |_| {
        if busy() {
            return;
        }
        let Some(ref token) = auth.token_value() else {
            toaster.error("请先登录后再借阅");
            return;
        };
        let Some(uid) = user_id else {
            toaster.error("无法获取用户信息");
            return;
        };
        busy.set(true);
        let token = token.clone();
        spawn(async move {
            match api::borrow_book(&token, uid, id).await {
                Ok(msg) => {
                    toaster.success(msg);
                    on_changed.call(());
                }
                Err(err) => toaster.error(err),
            }
            busy.set(false);
        });
    };

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
            match api::return_book(&token, uid, id).await {
                Ok(msg) => {
                    toaster.success(msg);
                    on_changed.call(());
                }
                Err(err) => toaster.error(err),
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
        spawn(async move {
            match api::delete_book(&token, id).await {
                Ok(msg) => {
                    toaster.success(msg);
                    on_deleted.call(());
                }
                Err(err) => toaster.error(err),
            }
            busy.set(false);
        });
    };

    let bar_color = if available_pct > 0.5 {
        "bg-emerald-400"
    } else if available_pct > 0.0 {
        "bg-amber-400"
    } else {
        "bg-rose-400"
    };
    let avail_color = if book.available > 0 {
        "text-emerald-600"
    } else {
        "text-rose-500"
    };

    rsx! {
        div { class: "group flex flex-col rounded-xl border border-slate-200 bg-white p-4 shadow-sm shadow-slate-900/5 transition hover:shadow-md hover:border-slate-300",
            div { class: "flex gap-4",
                div { class: "grid place-items-center w-14 h-20 shrink-0 rounded-lg bg-gradient-to-br {grad} text-white text-2xl font-bold shadow-inner",
                    "{initial}"
                }
                div { class: "min-w-0 flex-1",
                    h3 { class: "font-semibold text-slate-900 leading-snug line-clamp-2", "{book.name}" }
                    p { class: "text-sm text-slate-500 mt-1 truncate", "{book.author}" }
                    if !book.description.is_empty() {
                        p { class: "text-xs text-slate-400 mt-1 line-clamp-2", "{book.description}" }
                    }
                    div { class: "flex items-center gap-2 mt-2",
                        span { class: "rounded-md bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-500", "#{book.id}" }
                        div { class: "flex items-center gap-1.5",
                            div { class: "w-16 h-1.5 rounded-full bg-slate-200 overflow-hidden",
                                div {
                                    class: "h-full rounded-full transition-all {bar_color}",
                                    width: "{available_pct * 100.0}%",
                                }
                            }
                            span { class: "text-xs {avail_color}",
                                if book.available > 0 { "{book.available}/{book.total} 可借" } else { "已借完" }
                            }
                        }
                    }
                }
            }

            div { class: "flex items-center gap-2 mt-4 pt-3 border-t border-slate-100",
                if borrowed {
                    span { class: "inline-flex items-center gap-1 text-xs font-medium text-emerald-600",
                        span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500" }
                        "已借阅"
                    }
                    div { class: "flex-1" }
                    button {
                        class: "rounded-lg border border-slate-200 px-3.5 py-1.5 text-sm font-medium text-slate-700 transition hover:bg-slate-50 disabled:opacity-50",
                        disabled: busy(),
                        onclick: do_return,
                        "归还"
                    }
                } else {
                    div { class: "flex-1" }
                    if auth.is_authed() {
                        button {
                            class: "rounded-lg bg-indigo-600 px-3.5 py-1.5 text-sm font-medium text-white transition hover:bg-indigo-700 disabled:opacity-50",
                            disabled: busy() || book.available < 1,
                            onclick: do_borrow,
                            if book.available < 1 { "不可借" } else { "借阅" }
                        }
                    } else {
                        button {
                            class: "rounded-lg border border-slate-200 px-3.5 py-1.5 text-sm font-medium text-slate-500 transition hover:bg-slate-50",
                            onclick: move |_| { nav.push(Route::Login {}); },
                            "登录借阅"
                        }
                    }
                }

                if auth.is_admin() {
                    div { class: "flex items-center gap-1 ml-1 pl-2 border-l border-slate-100",
                        button {
                            class: "rounded-lg p-1.5 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700",
                            title: "编辑",
                            onclick: move |_| on_edit.call(edit_book.clone()),
                            "✎"
                        }
                        button {
                            class: "rounded-lg p-1.5 text-slate-400 transition hover:bg-rose-50 hover:text-rose-600 disabled:opacity-50",
                            title: "删除",
                            disabled: busy(),
                            onclick: do_delete,
                            "🗑"
                        }
                    }
                }
            }
        }
    }
}
