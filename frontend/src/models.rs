use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub author: String,
    #[serde(default)]
    pub description: String,
    pub total: i32,
    pub available: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BorrowedBook {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub author: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub permission: String,
    #[serde(default)]
    pub borrowed_books: Vec<BorrowedBook>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.permission == "admin"
    }

    pub fn has_borrowed(&self, id: i32) -> bool {
        self.borrowed_books.iter().any(|b| b.id == id)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BookListResponse {
    pub items: Vec<Book>,
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserBrief {
    pub id: i32,
    pub username: String,
    pub permission: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BorrowRecord {
    pub user_id: i32,
    pub username: String,
    pub book_id: i32,
    pub book_name: String,
}
