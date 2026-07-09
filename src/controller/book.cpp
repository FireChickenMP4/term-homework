#include "models/models.h"
#include <drogon/drogon.h>
#include "utils/response.h"
#include "auth/AuthFilter.h"

void registerBookRoutes(drogon::HttpAppFramework &app)
{
    app.registerHandler("/books/search",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto keyword = req->getParameter("q");
                            auto books = Book::search(keyword);
                            Json::Value json(Json::arrayValue);
                            for (auto &b : books)
                            {
                                Json::Value v;
                                v["id"] = b.id;
                                v["name"] = b.name;
                                v["author"] = b.author;
                                v["description"] = b.description;
                                v["total"] = b.total;
                                v["available"] = b.available;
                                json.append(v);
                            }
                            cb(drogon::HttpResponse::newHttpJsonResponse(json));
                        },
                        {drogon::Get});

    app.registerHandler("/books",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto page = std::stoi(req->getParameter("page").empty() ? "1" : req->getParameter("page"));
                            auto limit = std::stoi(req->getParameter("limit").empty() ? "20" : req->getParameter("limit"));
                            auto total = Book::countAll();
                            auto books = Book::getAll(page, limit);
                            Json::Value items(Json::arrayValue);
                            for (auto &b : books)
                            {
                                Json::Value v;
                                v["id"] = b.id;
                                v["name"] = b.name;
                                v["author"] = b.author;
                                v["description"] = b.description;
                                v["total"] = b.total;
                                v["available"] = b.available;
                                items.append(v);
                            }
                            Json::Value json;
                            json["items"] = items;
                            json["total"] = total;
                            json["page"] = page;
                            json["limit"] = limit;
                            cb(drogon::HttpResponse::newHttpJsonResponse(json));
                        },
                        {drogon::Get});

    app.registerHandler("/books/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            try
                            {
                                auto book = Book::getById(id);
                                Json::Value v;
                                v["id"] = book.id;
                                v["name"] = book.name;
                                v["author"] = book.author;
                                v["description"] = book.description;
                                v["total"] = book.total;
                                v["available"] = book.available;
                                cb(drogon::HttpResponse::newHttpJsonResponse(v));
                            }
                            catch (const std::exception &e)
                            {
                                cb(err(e.what(), 404));
                            }
                        },
                        {drogon::Get});

    app.registerHandler("/books",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb)
                        {
                            auto json = req->getJsonObject();
                            auto id = Book::add((*json)["name"].asString(),
                                                (*json)["author"].asString(),
                                                (*json)["description"].asString(),
                                                (*json).get("total", 1).asInt());
                            Json::Value v;
                            v["id"] = id;
                            auto resp = drogon::HttpResponse::newHttpJsonResponse(v);
                            resp->setStatusCode(drogon::k201Created);
                            cb(resp);
                        },
                        {drogon::Post, "AuthFilter"});

    app.registerHandler("/books/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            auto json = req->getJsonObject();
                            Book::update(id,
                                         (*json)["name"].asString(),
                                         (*json)["author"].asString(),
                                         (*json)["description"].asString(),
                                         (*json).get("total", 1).asInt());
                            cb(drogon::HttpResponse::newHttpResponse());
                        },
                        {drogon::Put, "AuthFilter"});

    app.registerHandler("/books/{id}",
                        [](const drogon::HttpRequestPtr &req,
                           std::function<void(const drogon::HttpResponsePtr &)> &&cb, int id)
                        {
                            try
                            {
                                Book::remove(id);
                                cb(drogon::HttpResponse::newHttpResponse());
                            }
                            catch (const std::exception &e)
                            {
                                cb(err(e.what()));
                            }
                        },
                        {drogon::Delete, "AuthFilter"});
}
