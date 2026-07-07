use std::collections::HashMap;

use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::models::{Book, User};

const TOKEN_KEY: &str = "library_token";

// ---------------------------------------------------------------------------
// Authentication
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Auth {
    pub token: Signal<Option<String>>,
    pub user: Signal<Option<User>>,
}

impl Auth {
    pub fn token_value(&self) -> Option<String> {
        self.token.read().clone()
    }

    pub fn is_authed(&self) -> bool {
        self.token.read().is_some()
    }

    pub fn user_value(&self) -> Option<User> {
        self.user.read().clone()
    }

    pub fn is_admin(&self) -> bool {
        self.user.read().as_ref().is_some_and(User::is_admin)
    }

    pub fn set_session(&mut self, token: String, user: User) {
        let _ = LocalStorage::set(TOKEN_KEY, &token);
        self.token.set(Some(token));
        self.user.set(Some(user));
    }

    pub fn set_user(&mut self, user: User) {
        self.user.set(Some(user));
    }

    pub fn logout(&mut self) {
        LocalStorage::delete(TOKEN_KEY);
        self.token.set(None);
        self.user.set(None);
    }
}

// ---------------------------------------------------------------------------
// Book cache (resolve borrowed-book ids to titles across pages)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct BookCache {
    map: Signal<HashMap<i32, Book>>,
}

impl BookCache {
    pub fn insert_many(&mut self, books: &[Book]) {
        let mut map = self.map.write();
        for book in books {
            map.insert(book.id, book.clone());
        }
    }
}

// ---------------------------------------------------------------------------
// Toast notifications
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Clone, Copy)]
pub struct Toaster {
    items: Signal<Vec<Toast>>,
    next_id: Signal<u64>,
}

impl Toaster {
    pub fn items(&self) -> Signal<Vec<Toast>> {
        self.items
    }

    pub fn push(&mut self, kind: ToastKind, message: impl Into<String>) {
        let id = {
            let current = (self.next_id)();
            self.next_id.set(current + 1);
            current
        };
        self.items.write().push(Toast {
            id,
            message: message.into(),
            kind,
        });
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.push(ToastKind::Success, message);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.push(ToastKind::Error, message);
    }

    pub fn dismiss(&mut self, id: u64) {
        self.items.write().retain(|t| t.id != id);
    }
}

// ---------------------------------------------------------------------------
// Providers & hooks
// ---------------------------------------------------------------------------

/// Create and provide all global state. Returns the [`Auth`] handle so the
/// root component can restore a saved session.
pub fn provide_state() -> Auth {
    let token = use_signal(|| LocalStorage::get::<String>(TOKEN_KEY).ok());
    let user = use_signal(|| Option::<User>::None);
    let auth = Auth { token, user };
    use_context_provider(|| auth);

    let toaster = Toaster {
        items: use_signal(Vec::<Toast>::new),
        next_id: use_signal(|| 0u64),
    };
    use_context_provider(|| toaster);

    let cache = BookCache {
        map: use_signal(HashMap::<i32, Book>::new),
    };
    use_context_provider(|| cache);

    auth
}

pub fn use_auth() -> Auth {
    use_context::<Auth>()
}

pub fn use_toaster() -> Toaster {
    use_context::<Toaster>()
}

pub fn use_book_cache() -> BookCache {
    use_context::<BookCache>()
}
