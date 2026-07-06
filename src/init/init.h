#pragma once
#include <drogon/HttpAppFramework.h>
#include <drogon/plugins/AccessLogger.h>

namespace Plugins {
    inline void registerPlugins(drogon::HttpAppFramework& app) {
        // config.json 已有 plugins 配置时这里不重复添加
        Json::Value plugins;
        plugins[0]["name"] = "drogon::plugin::AccessLogger";
        app.addPlugins(plugins);
    }
}

namespace Routes {
    void registerRoutes(drogon::HttpAppFramework& app);
}

void init(drogon::HttpAppFramework& app);