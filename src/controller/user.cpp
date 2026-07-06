#include <drogon/drogon.h>
#include "models/models.h"
#include "auth/AuthFilter.h"
#include "utils/response.h"
#include "auth/jwt.h"

void registerUserRoutes(drogon::HttpAppFramework& app) {
    app.registerHandler("/login",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        auto json = req->getJsonObject();
        if (!json) { cb(err("Invalid JSON")); return; }
        try {
            auto user = User::getByUsername((*json)["username"].asString());
            if (!User::verifyPassword(user.id, (*json)["password"].asString())) {
                cb(err("Wrong password", 401)); return;
            }
            Json::Value token;
            token["token"] = jwt::create(user.id,
                user.permission == User::Permission::Admin ? "admin" : "user");
            cb(drogon::HttpResponse::newHttpJsonResponse(token));
        } catch (...) {
            cb(err("User not found", 401));
        }
    }, {drogon::Post});
    app.registerHandler("/users",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        auto users = User::getAll();
        Json::Value json(Json::arrayValue);
        for (auto& u : users) {
            Json::Value v;
            v["id"] = u.id;
            v["username"] = u.username;
            v["permission"] = u.permission == User::Permission::Admin ? "admin" : "user";
            json.append(v);
        }
        cb(drogon::HttpResponse::newHttpJsonResponse(json));
    }, {drogon::Get, "AdminFilter"});

    app.registerHandler("/users/{id}",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb, int id) {
        auto user = User::getById(id);
        Json::Value v;
        v["id"] = user.id;
        v["username"] = user.username;
        v["permission"] = user.permission == User::Permission::Admin ? "admin" : "user";
        cb(drogon::HttpResponse::newHttpJsonResponse(v));
    }, {drogon::Get});

    app.registerHandler("/users/{id}",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb, int id) {
        auto json = req->getJsonObject();
        if (!User::verifyPassword(id, (*json)["old_password"].asString())) {
            cb(err("Wrong password"));
            return;
        }
        User::update(id, (*json)["username"].asString(),
                     (*json)["password"].asString());
        cb(drogon::HttpResponse::newHttpResponse());
    }, {drogon::Put, "AuthFilter"});

    app.registerHandler("/users/{id}/permission",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb, int id) {
        if (id == 1) { cb(err("Cannot modify super admin", 403)); return; }
        auto json = req->getJsonObject();
        auto perm = (*json)["permission"].asString() == "admin"
                    ? User::Permission::Admin : User::Permission::User;
        User::setPermission(id, perm);
        cb(drogon::HttpResponse::newHttpResponse());
    }, {drogon::Put, "SuperAdminFilter"});

    app.registerHandler("/users/{id}",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb, int id) {
        if (id == 1) { cb(err("Cannot delete super admin", 403)); return; }
        try {
            User::remove(id);
            cb(drogon::HttpResponse::newHttpResponse());
        } catch (const std::exception& e) {
            auto resp = drogon::HttpResponse::newHttpResponse();
            resp->setStatusCode(drogon::k400BadRequest);
            resp->setBody(e.what());
            cb(resp);
        }
    }, {drogon::Delete, "SuperAdminFilter"});

    app.registerHandler("/users",
        [](const drogon::HttpRequestPtr& req,
           std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
        auto json = req->getJsonObject();
        auto perm = (*json)["permission"].asString() == "admin"
                    ? User::Permission::Admin : User::Permission::User;
        auto id = User::add((*json)["username"].asString(),
                            (*json)["password"].asString(), perm);
        Json::Value v; v["id"] = id;
        auto resp = drogon::HttpResponse::newHttpJsonResponse(v);
        resp->setStatusCode(drogon::k201Created);
        cb(resp);
    }, {drogon::Post});
}
