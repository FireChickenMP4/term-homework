use gloo_net::http::Request;
use serde_json::json;

use crate::models::{Book, BookListResponse, BorrowRecord, User, UserBrief};

pub const BASE: &str = "/api";

fn bearer(token: &str) -> String {
    format!("Bearer {token}")
}

fn net_err(e: gloo_net::Error) -> String {
    format!("无法连接服务器：{e}")
}

async fn parse_error(resp: gloo_net::http::Response) -> String {
    let status = resp.status();
    match resp.text().await {
        Ok(text) => {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                v.get("error")
                    .and_then(|m| m.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("请求失败（HTTP {status}）"))
            } else {
                format!("请求失败（HTTP {status}）")
            }
        }
        Err(e) => format!("无法解析响应：{e}"),
    }
}

pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let body = json!({ "username": username, "password": password });
    let resp = Request::post(&format!("{BASE}/login"))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.map_err(net_err)?;
        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))?;
        v.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "响应缺少 token".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn refresh(token: &str) -> Result<String, String> {
    let resp = Request::post(&format!("{BASE}/refresh"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.map_err(net_err)?;
        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))?;
        v.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "响应缺少 token".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn register(username: &str, password: &str) -> Result<String, String> {
    let body = json!({ "username": username, "password": password });
    let resp = Request::post(&format!("{BASE}/register"))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("注册成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn me(token: &str) -> Result<User, String> {
    let resp = Request::get(&format!("{BASE}/me"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.map_err(net_err)?;
        serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn search_books(keyword: &str, page: i32) -> Result<BookListResponse, String> {
    let kw = keyword.trim();

    if kw.is_empty() {
        let url = format!("{BASE}/books?page={page}&limit=9");
        let resp = Request::get(&url).timeout(TIMEOUT).send().await.map_err(net_err)?;
        if resp.ok() {
            let text = resp.text().await.map_err(net_err)?;
            serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))
        } else {
            Err(parse_error(resp).await)
        }
    } else {
        let url = format!("{BASE}/books/search?q={}", urlencoding::encode(kw));
        let resp = Request::get(&url).timeout(TIMEOUT).send().await.map_err(net_err)?;
        if resp.ok() {
            let text = resp.text().await.map_err(net_err)?;
            let books: Vec<Book> =
                serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))?;
            let total = books.len() as i32;
            Ok(BookListResponse {
                items: books,
                total,
                page: 1,
                limit: total,
            })
        } else {
            Err(parse_error(resp).await)
        }
    }
}

pub async fn create_book(
    token: &str,
    name: &str,
    author: &str,
    description: &str,
    total: i32,
) -> Result<String, String> {
    let body =
        json!({ "name": name, "author": author, "description": description, "total": total });
    let resp = Request::post(&format!("{BASE}/books"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("图书创建成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn update_book(
    token: &str,
    id: i32,
    name: &str,
    author: &str,
    description: &str,
    total: i32,
) -> Result<String, String> {
    let body =
        json!({ "name": name, "author": author, "description": description, "total": total });
    let resp = Request::put(&format!("{BASE}/books/{id}"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("图书更新成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn update_profile(
    token: &str,
    user_id: i32,
    old_password: &str,
    username: &str,
    password: &str,
) -> Result<String, String> {
    let body = json!({ "old_password": old_password, "username": username, "password": password });
    let resp = Request::put(&format!("{BASE}/users/{user_id}"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("个人信息已更新".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn self_delete(token: &str, user_id: i32) -> Result<String, String> {
    let resp = Request::delete(&format!("{BASE}/users/{user_id}"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.unwrap_or_default();
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            Ok(v.get("msg")
                .and_then(|m| m.as_str())
                .unwrap_or("账号已注销")
                .to_string())
        } else {
            Ok("账号已注销".into())
        }
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn delete_book(token: &str, id: i32) -> Result<String, String> {
    let resp = Request::delete(&format!("{BASE}/books/{id}"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("图书删除成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn borrow_book(token: &str, user_id: i32, book_id: i32) -> Result<String, String> {
    let body = json!({ "user_id": user_id, "book_id": book_id });
    let resp = Request::post(&format!("{BASE}/borrow"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("借阅成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn list_users(token: &str) -> Result<Vec<UserBrief>, String> {
    let resp = Request::get(&format!("{BASE}/users"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.map_err(net_err)?;
        serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn set_permission(token: &str, user_id: i32, permission: &str) -> Result<String, String> {
    let body = json!({ "permission": permission });
    let resp = Request::put(&format!("{BASE}/users/{user_id}/permission"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("权限已更新".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn admin_delete_user(token: &str, user_id: i32) -> Result<String, String> {
    let resp = Request::delete(&format!("{BASE}/users/{user_id}"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("用户已删除".into())
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn list_borrowed(token: &str) -> Result<Vec<BorrowRecord>, String> {
    let resp = Request::get(&format!("{BASE}/admin/borrowed"))
        .header("Authorization", &bearer(token))
        
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        let text = resp.text().await.map_err(net_err)?;
        serde_json::from_str(&text).map_err(|e| format!("解析失败：{e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

pub async fn return_book(token: &str, user_id: i32, book_id: i32) -> Result<String, String> {
    let body = json!({ "user_id": user_id, "book_id": book_id });
    let resp = Request::post(&format!("{BASE}/return"))
        .header("Authorization", &bearer(token))
        
        .json(&body)
        .map_err(net_err)?
        .send()
        .await
        .map_err(net_err)?;
    if resp.ok() {
        Ok("归还成功".into())
    } else {
        Err(parse_error(resp).await)
    }
}
