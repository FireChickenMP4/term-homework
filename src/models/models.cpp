#include "models.h"
#include "utils/hash.h"
#include "utils/config.h"
#include <drogon/drogon.h>
#include <mysql/mysql.h>

static auto db() { return drogon::app().getDbClient(); }

static Book rowToBook(const drogon::orm::Row& row) {
    Book b;
    b.id = row["id"].as<int>();
    b.name = row["name"].as<std::string>();
    b.author = row["author"].as<std::string>();
    b.description = row["description"].as<std::string>();
    b.total = row["total"].as<int>();
    b.available = row["available"].as<int>();
    return b;
}

static User rowToUser(const drogon::orm::Row& row) {
    User u;
    u.id = row["id"].as<int>();
    u.username = row["username"].as<std::string>();
    u.password = row["password"].as<std::string>();
    u.permission = row["permission"].as<std::string>() == "admin"
                   ? User::Permission::Admin : User::Permission::User;
    return u;
}

// ─── Book ──────────────────────────────

std::vector<Book> Book::getAll(int page, int limit) {
    auto offset = (page - 1) * limit;
    auto result = db()->execSqlSync(
        "SELECT * FROM books ORDER BY id LIMIT ? OFFSET ?",
        limit, offset);
    std::vector<Book> books;
    for (auto& row : result) books.push_back(rowToBook(row));
    return books;
}

std::vector<Book> Book::search(const std::string& keyword) {
    if (keyword.empty()) return getAll();
    auto kw = "%" + keyword + "%";
    auto result = db()->execSqlSync(
        "SELECT * FROM books WHERE name LIKE ? OR author LIKE ? OR description LIKE ?",
        kw, kw, kw);
    std::vector<Book> books;
    for (auto& row : result) books.push_back(rowToBook(row));
    return books;
}

Book Book::getById(int id) {
    auto result = db()->execSqlSync("SELECT * FROM books WHERE id = ?",
                                    std::to_string(id));
    if (result.empty()) throw std::runtime_error("Book not found");
    return rowToBook(result[0]);
}

int Book::countAll() {
    auto r = db()->execSqlSync("SELECT COUNT(*) FROM books");
    return r[0][0].as<int>();
}

int Book::add(const std::string& name, const std::string& author,
              const std::string& description, int total) {
    db()->execSqlSync(
        "INSERT INTO books (name, author, description, total, available) "
        "VALUES (?, ?, ?, ?, ?)", name, author, description, total, total);
    auto r = db()->execSqlSync("SELECT LAST_INSERT_ID()");
    return r[0]["LAST_INSERT_ID()"].as<int>();
}

void Book::update(int id, const std::string& name,
                  const std::string& author, const std::string& description,
                  int total) {
    db()->execSqlSync(
        "UPDATE books SET name=?, author=?, description=?, total=? WHERE id=?",
        name, author, description, total, std::to_string(id));
}

void Book::remove(int id) {
    auto check = db()->execSqlSync(
        "SELECT 1 FROM borrowed_books WHERE book_id = ? LIMIT 1",
        std::to_string(id));
    if (!check.empty())
        throw std::runtime_error("Book is currently borrowed, cannot delete");
    db()->execSqlSync("DELETE FROM books WHERE id = ?", std::to_string(id));
}

// ─── User ──────────────────────────────

User User::getById(int id) {
    auto result = db()->execSqlSync("SELECT * FROM users WHERE id = ?",
                                    std::to_string(id));
    if (result.empty()) throw std::runtime_error("User not found");
    return rowToUser(result[0]);
}

User User::getByUsername(const std::string& username) {
    auto result = db()->execSqlSync(
        "SELECT * FROM users WHERE username = ?", username);
    if (result.empty()) throw std::runtime_error("User not found");
    return rowToUser(result[0]);
}

int User::add(const std::string& username, const std::string& password,
              Permission permission) {
    auto perm = permission == Permission::Admin ? "admin" : "user";
    auto salt = randomSalt();
    db()->execSqlSync(
        "INSERT INTO users (username, password, permission, salt) VALUES (?, ?, ?, ?)",
        username, hashPassword(password, salt), perm, salt);
    auto r = db()->execSqlSync("SELECT LAST_INSERT_ID()");
    return r[0][0].as<int>();
}

bool User::verifyPassword(int id, const std::string& password) {
    auto result = db()->execSqlSync("SELECT password, salt FROM users WHERE id = ?", std::to_string(id));
    if (result.empty()) return false;
    auto row = result[0];
    return row["password"].as<std::string>() == hashPassword(password, row["salt"].as<std::string>());
}

void User::update(int id, const std::string& username,
                  const std::string& password) {
    auto salt = randomSalt();
    db()->execSqlSync("UPDATE users SET username=?, password=?, salt=? WHERE id=?",
                      username, hashPassword(password, salt), salt, std::to_string(id));
}

void User::setPermission(int id, Permission permission) {
    auto perm = permission == Permission::Admin ? "admin" : "user";
    db()->execSqlSync("UPDATE users SET permission=? WHERE id=?",
                      perm, std::to_string(id));
}

void User::remove(int id) {
    auto check = db()->execSqlSync(
        "SELECT 1 FROM borrowed_books WHERE user_id = ? LIMIT 1",
        std::to_string(id));
    if (!check.empty())
        throw std::runtime_error("User has borrowed books, cannot delete");
    db()->execSqlSync("DELETE FROM users WHERE id = ?", std::to_string(id));
}

std::vector<User> User::getAll() {
    auto result = db()->execSqlSync("SELECT * FROM users");
    std::vector<User> users;
    for (auto& row : result) users.push_back(rowToUser(row));
    return users;
}

// ─── Library ───────────────────────────

void Library::borrowBook(int userId, int bookId) {
    auto res = db()->execSqlSync(
        "UPDATE books SET available = available - 1 "
        "WHERE id = ? AND available > 0",
        std::to_string(bookId));
    if (res.affectedRows() == 0)
        throw std::runtime_error("No copies available");

    try {
        db()->execSqlSync(
            "INSERT INTO borrowed_books (user_id, book_id) VALUES (?, ?)",
            std::to_string(userId), std::to_string(bookId));
    } catch (...) {
        db()->execSqlSync(
            "UPDATE books SET available = available + 1 WHERE id = ?",
            std::to_string(bookId));
        throw std::runtime_error("Already borrowed this book");
    }
}

void Library::returnBook(int userId, int bookId) {
    auto res = db()->execSqlSync(
        "DELETE FROM borrowed_books WHERE user_id = ? AND book_id = ?",
        std::to_string(userId), std::to_string(bookId));
    if (res.affectedRows() == 0)
        throw std::runtime_error("No such borrow record");

    db()->execSqlSync(
        "UPDATE books SET available = available + 1 "
        "WHERE id = ? AND available < total",
        std::to_string(bookId));
}

Json::Value Library::getAllBorrowed() {
    auto result = db()->execSqlSync(
        "SELECT bb.user_id, u.username, b.id as book_id, b.name as book_name "
        "FROM borrowed_books bb "
        "JOIN users u ON bb.user_id = u.id "
        "JOIN books b ON bb.book_id = b.id "
        "ORDER BY bb.user_id");
    Json::Value json(Json::arrayValue);
    for (auto& row : result) {
        Json::Value v;
        v["user_id"] = row["user_id"].as<int>();
        v["username"] = row["username"].as<std::string>();
        v["book_id"] = row["book_id"].as<int>();
        v["book_name"] = row["book_name"].as<std::string>();
        json.append(v);
    }
    return json;
}

std::vector<Book> Library::getBorrowedBooks(int userId) {
    auto result = db()->execSqlSync(
        "SELECT b.* FROM books b "
        "JOIN borrowed_books bb ON b.id = bb.book_id "
        "WHERE bb.user_id = ?", std::to_string(userId));
    std::vector<Book> books;
    for (auto& row : result) books.push_back(rowToBook(row));
    return books;
}
