#pragma once
#include <drogon/HttpAppFramework.h>
#include <drogon/plugins/AccessLogger.h>

namespace Plugins {
    inline void registerPlugins(drogon::HttpAppFramework& app) {
        Json::Value plugins;
        plugins[0]["name"] = "drogon::plugin::AccessLogger";
        // plugins[0]["config"]["log_path"] = "./";
        // plugins[0]["config"]["log_format"] = "$method $url $status $body_bytes_sent";
        app.addPlugins(plugins);
    }
}

namespace Routes {
    void registerRoutes(drogon::HttpAppFramework& app);
}

void init(drogon::HttpAppFramework& app) {
    app.addDbClient(drogon::orm::MysqlConfig{
        .host = "127.0.0.1",
        .port = 3306,
        .databaseName = "lib",
        .username = "lib",
        .password = "lib",
        .connectionNumber = 1,
        .name = "default",
        .isFast = false,
        .characterSet = "utf8mb4",
        .timeout = 1.0
    });
    // app.registerHandler("/books", [](const drogon::HttpRequestPtr& req,
    //     std::function<void(const drogon::HttpResponsePtr&)>&& cb) {
    //     auto db = drogon::app().getDbClient();
    //     auto result = db->execSqlSync("SELECT * FROM books");
    //     Json::Value json(Json::arrayValue);
    //     for (auto& row : result) {
    //         Json::Value book;
    //         book["id"] = row["id"].as<int>();
    //         book["name"] = row["name"].as<std::string>();
    //         json.append(book);
    //     }
    //     cb(drogon::HttpResponse::newHttpJsonResponse(json));
    // });
    Plugins::registerPlugins(app);
    app.setLogLevel(trantor::Logger::kInfo);
    Routes::registerRoutes(app);
}