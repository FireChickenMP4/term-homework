#include "init.h"

void registerBookRoutes(drogon::HttpAppFramework&);
void registerUserRoutes(drogon::HttpAppFramework&);
void registerLibRoutes(drogon::HttpAppFramework&);

namespace Routes {
void registerRoutes(drogon::HttpAppFramework& app) {
    app.registerHandler("/health",[](const drogon::HttpRequestPtr& req,
        std::function<void(const drogon::HttpResponsePtr&)>&& callback)
    {
        auto resp = drogon::HttpResponse::newHttpResponse();
        resp->setBody("Healthy");
        callback(resp);
    });

    registerBookRoutes(app);
    registerUserRoutes(app);
    registerLibRoutes(app);
}
}
