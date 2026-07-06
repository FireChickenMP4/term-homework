#pragma once
#include <string>
#include <vector>

class Library {
public:

private:
    std::vector<Book> books;
    std::vector<User> users;
};

class Book {
public:

private:
    std::string name;
    std::string author;
    std::string description;
};

class User {
public:
    enum class Permission {
        User,
        Admin
    };
private:
    std::string username;
    std::string password;
    Permission permission;
    std::vector<const Book*> borrowedBooks;
};