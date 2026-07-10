#include <drogon/drogon.h>
#include "models/models.h"
#include "auth/AuthFilter.h"
#include "utils/response.h"
#include "auth/jwt.h"
#include <json/value.h>

void registerUserRoutes(drogon::HttpAppFramework &app)
{
    app.registerHandler("/api/refresh",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto jwt = getCurrentUser(req);
                            if (jwt.isNull())
                            {
                                cb(err("请先登录", 401));
                                return;
                            }
                            int userId = jwt["user_id"].asInt();
                            std::string permission = jwt["permission"].asString();
                            Json::Value token;
                            token["token"] = jwt::create(userId, permission);
                            cb(drogon::HttpResponse::newHttpJsonResponse(token));
                        },
                        {drogon::Post, "AuthFilter"});
    app.registerHandler("/api/login",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto json = req->getJsonObject();
                            if (!json)
                            {
                                cb(err("用户名或密码错误", 401));
                                return;
                            }
                            try
                            {
                                auto user = User::getByUsername((*json)["username"].asString());
                                if (!User::verifyPassword(user.id, (*json)["password"].asString()))
                                {
                                    cb(err("用户名或密码错误", 401));
                                    return;
                                }
                                Json::Value token;
                                token["token"] = jwt::create(user.id,
                                                             user.permission == User::Permission::Admin ? "admin" : "user");
                                cb(drogon::HttpResponse::newHttpJsonResponse(token));
                            }
                            catch (...)
                            {
                                cb(err("用户名或密码错误", 401));
                            }
                        },
                        {drogon::Post});
    // ── /me (get current user with borrowed books) ──
    app.registerHandler("/api/me",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto jwt = getCurrentUser(req);
                            if (jwt.isNull())
                            {
                                cb(err("未登录", 401));
                                return;
                            }
                            int userId = jwt["user_id"].asInt();
                            auto user = User::getById(userId);
                            auto books = Library::getBorrowedBooks(userId);
                            Json::Value v;
                            v["id"] = user.id;
                            v["username"] = user.username;
                            v["permission"] = user.permission == User::Permission::Admin ? "admin" : "user";
                            Json::Value arr(Json::arrayValue);
                            for (auto &b : books)
                            {
                                Json::Value bv;
                                bv["id"] = b.id;
                                bv["name"] = b.name;
                                bv["author"] = b.author;
                                arr.append(bv);
                            }
                            v["borrowed_books"] = arr;
                            cb(drogon::HttpResponse::newHttpJsonResponse(v));
                        },
                        {drogon::Get, "AuthFilter"});

    // ── /register (create user, return success message) ──
    app.registerHandler("/api/register",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto json = req->getJsonObject();
                            if (!json || !(*json).isMember("username") || !(*json).isMember("password"))
                            {
                                cb(err("请填写用户名和密码"));
                                return;
                            }
                            try
                            {
                                User::getByUsername((*json)["username"].asString());
                                cb(err("用户名已存在"));
                                return;
                            }
                            catch (...)
                            {
                            }
                            auto name = (*json)["username"].asString();
                            if (name.length() < 2 || name.length() > 20)
                            {
                                cb(err("用户名长度需 2-20 位"));
                                return;
                            }
                            auto pwd = (*json)["password"].asString();
                            if (pwd.length() < 6)
                            {
                                cb(err("密码长度至少 6 位"));
                                return;
                            }
                            User::add(name, pwd, User::Permission::User);
                            Json::Value v;
                            v["msg"] = "注册成功";
                            cb(drogon::HttpResponse::newHttpJsonResponse(v));
                        },
                        {drogon::Post});

    app.registerHandler("/api/users",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto users = User::getAll();
                            Json::Value json(Json::arrayValue);
                            for (auto &u : users)
                            {
                                Json::Value v;
                                v["id"] = u.id;
                                v["username"] = u.username;
                                v["permission"] = u.permission == User::Permission::Admin ? "admin" : "user";
                                json.append(v);
                            }
                            cb(drogon::HttpResponse::newHttpJsonResponse(json));
                        },
                        {drogon::Get, "AdminFilter"});

    app.registerHandler("/api/users/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            auto jwt = getCurrentUser(req);
                            if (jwt.isNull())
                            {
                                cb(err("未登录", 401));
                                return;
                            }
                            int currentId = jwt["user_id"].asInt();
                            std::string currentPerm = jwt["permission"].asString();
                            if (currentId != id && currentPerm != "admin")
                            {
                                cb(err("无权访问", 403));
                                return;
                            }
                            auto user = User::getById(id);
                            Json::Value v;
                            v["id"] = user.id;
                            v["username"] = user.username;
                            v["permission"] = user.permission == User::Permission::Admin ? "admin" : "user";
                            cb(drogon::HttpResponse::newHttpJsonResponse(v));
                        },
                        {drogon::Get, "AuthFilter"});

    app.registerHandler("/api/users/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            auto json = req->getJsonObject();
                            if (!User::verifyPassword(id, (*json)["old_password"].asString()))
                            {
                                cb(err("当前密码错误", 401));
                                return;
                            }
                            {
                                auto name = (*json)["username"].asString();
                                if (name.length() < 2 || name.length() > 20)
                                {
                                    cb(err("用户名长度需 2-20 位"));
                                    return;
                                }
                                auto newPwd = (*json)["password"].asString();
                                if (!newPwd.empty() && newPwd.length() < 6)
                                {
                                    cb(err("新密码长度至少 6 位"));
                                    return;
                                }
                            }
                            User::update(id, (*json)["username"].asString(), (*json)["password"].asString());
                            cb(drogon::HttpResponse::newHttpResponse());
                        },
                        {drogon::Put, "AuthFilter"});

    app.registerHandler("/api/users/{id}/permission",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            if (id == 1)
                            {
                                cb(err("不能修改初始管理员", 403));
                                return;
                            }
                            try
                            {
                                auto target = User::getById(id);
                                auto cur = getCurrentUser(req);
                                bool isSuper = cur["user_id"].asInt() == 1;
                                if (target.permission == User::Permission::Admin && !isSuper)
                                {
                                    cb(err("不能修改其他管理员", 403));
                                    return;
                                }
                            }
                            catch (...)
                            {
                                cb(err("用户不存在", 404));
                                return;
                            }
                            auto json = req->getJsonObject();
                            auto newPerm = (*json)["permission"].asString();
                            auto cur = getCurrentUser(req);
                            if (cur["user_id"].asInt() != 1 && newPerm == "admin")
                            {
                                cb(err("无权提权为管理员", 403));
                                return;
                            }
                            auto perm = newPerm == "admin" ? User::Permission::Admin : User::Permission::User;
                            User::setPermission(id, perm);
                            cb(drogon::HttpResponse::newHttpResponse());
                        },
                        {drogon::Put, "AdminFilter"});

    app.registerHandler("/api/users/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            auto cur = getCurrentUser(req);
                            int curId = cur["user_id"].asInt();
                            bool isSelf = curId == id;
                            if (!isSelf)
                            {
                                // 管理员删其他人
                                try
                                {
                                    auto target = User::getById(id);
                                    bool isSuper = curId == 1;
                                    if (target.permission == User::Permission::Admin && !isSuper)
                                    {
                                        cb(err("不能删除其他管理员", 403));
                                        return;
                                    }
                                }
                                catch (...)
                                {
                                    cb(err("用户不存在", 404));
                                    return;
                                }
                            }
                            if (id == 1)
                            {
                                cb(err("不能删除初始管理员", 403));
                                return;
                            }
                            try
                            {
                                User::remove(id);
                                if (isSelf)
                                {
                                    Json::Value v;
                                    v["msg"] = "账号已注销";
                                    cb(drogon::HttpResponse::newHttpJsonResponse(v));
                                }
                                else
                                    cb(drogon::HttpResponse::newHttpResponse());
                            }
                            catch (const std::exception &e)
                            {
                                cb(err(e.what()));
                            }
                        },
                        {drogon::Delete, "AuthFilter"});

    app.registerHandler("/api/users",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto json = req->getJsonObject();
                            if (!json || !(*json).isMember("username") || !(*json).isMember("password"))
                            {
                                cb(err("请填写用户名和密码"));
                                return;
                            }
                            try
                            {
                                User::getByUsername((*json)["username"].asString());
                                cb(err("用户名已存在"));
                                return;
                            }
                            catch (...)
                            {
                            }
                            auto name = (*json)["username"].asString();
                            if (name.length() < 2 || name.length() > 20)
                            {
                                cb(err("用户名长度需 2-20 位"));
                                return;
                            }
                            auto pwd = (*json)["password"].asString();
                            if (pwd.length() < 6)
                            {
                                cb(err("密码长度至少 6 位"));
                                return;
                            }
                            auto id = User::add(name, pwd, User::Permission::User);
                            Json::Value v;
                            v["id"] = id;
                            auto resp = drogon::HttpResponse::newHttpJsonResponse(v);
                            resp->setStatusCode(drogon::k201Created);
                            cb(resp);
                        },
                        {drogon::Post});
}
