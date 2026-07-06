#include "init.h"

namespace Routes {
void registerRoutes(drogon::HttpAppFramework& app) {
    //路由注册
    {
        app.registerHandler("/health",[](const drogon::HttpRequestPtr& req,
            std::function<void(const drogon::HttpResponsePtr&)>&& callback)
        {
            auto resp = drogon::HttpResponse::newHttpResponse();
            resp->setBody("Healthy");
            callback(resp);
        });
    }
}
}