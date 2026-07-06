#pragma once
#include <string>
#include <vector>
#include <memory>
#include <json/value.h>

class Book {
public:
    int id;
    std::string name;
    std::string author;
    std::string description;
    int total;
    int available;

    static std::vector<Book> getAll(int page = 1, int limit = 20);
    static int countAll();
    static std::vector<Book> search(const std::string& keyword);
    static Book getById(int id);
    static int add(const std::string& name, const std::string& author,
                   const std::string& description, int total = 1);
    static void update(int id, const std::string& name,
                       const std::string& author, const std::string& description,
                       int total = 1);
    static void remove(int id);
};

class User {
public:
    enum class Permission { User, Admin };

    int id;
    std::string username;
    std::string password;
    Permission permission;

    static User getById(int id);
    static User getByUsername(const std::string& username);
    static int add(const std::string& username, const std::string& password,
                   Permission permission);
    static bool verifyPassword(int id, const std::string& password);
    static void update(int id, const std::string& username,
                       const std::string& password);
    static void setPermission(int id, Permission permission);
    static void remove(int id);
    static std::vector<User> getAll();
};

class Library {
public:
    static void borrowBook(int userId, int bookId);
    static void returnBook(int userId, int bookId);
    static std::vector<Book> getBorrowedBooks(int userId);
    static Json::Value getAllBorrowed();
};
