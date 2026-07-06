#include <drogon/drogon.h>
#include "models/models.h"
#include "utils/response.h"
#include "auth/AuthFilter.h"

void registerLibRoutes(drogon::HttpAppFramework& app) {
    app.registerHandler("/borrowed",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        cb(drogon::HttpResponse::newHttpJsonResponse(Library::getAllBorrowed()));
    }, {drogon::Get, "AdminFilter"});

    app.registerHandler("/borrow",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        auto json = req->getJsonObject();
        try {
            Library::borrowBook((*json)["user_id"].asInt(),
                                (*json)["book_id"].asInt());
            cb(drogon::HttpResponse::newHttpResponse());
        } catch (const std::exception& e) {
            cb(err(e.what()));
        }
    }, {drogon::Post, "AuthFilter"});

    app.registerHandler("/return",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        auto json = req->getJsonObject();
        try {
            Library::returnBook((*json)["user_id"].asInt(),
                                (*json)["book_id"].asInt());
            cb(drogon::HttpResponse::newHttpResponse());
        } catch (const std::exception& e) {
            cb(err(e.what()));
        }
    }, {drogon::Post, "AuthFilter"});

    app.registerHandler("/users/{id}/books",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb, int id) {
        auto books = Library::getBorrowedBooks(id);
        Json::Value json(Json::arrayValue);
        for (auto& b : books) {
            Json::Value v;
            v["id"] = b.id;
            v["name"] = b.name;
            json.append(v);
        }
        cb(drogon::HttpResponse::newHttpJsonResponse(json));
    }, {drogon::Get});
}
