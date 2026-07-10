#include "init.h"
#include <fstream>
#include <sstream>
#include <sys/stat.h>

void registerBookRoutes(drogon::HttpAppFramework &);
void registerUserRoutes(drogon::HttpAppFramework &);
void registerLibRoutes(drogon::HttpAppFramework &);

namespace Routes
{
    void registerRoutes(drogon::HttpAppFramework &app)
    {
        app.registerHandler("/api/health", [](const drogon::HttpRequestPtr &req,
                                          std::function<void(const drogon::HttpResponsePtr &)> &&callback)
                            {
            auto resp = drogon::HttpResponse::newHttpResponse();
            resp->setBody("Healthy");
            callback(resp); });

        // CORS preflight
        app.registerHandler("/api/*",
                            [](const drogon::HttpRequestPtr &,
                               std::function<void(const drogon::HttpResponsePtr &)> &&callback)
                            {
                                callback(drogon::HttpResponse::newHttpResponse());
                            },
                            {drogon::Options});

        registerBookRoutes(app);
        registerUserRoutes(app);
        registerLibRoutes(app);

        // SPA fallback — 浏览器刷新这些页面时发的是 GET，需返回 index.html
        struct stat st;
        bool hasFrontend = (stat("frontend/dist", &st) == 0 && S_ISDIR(st.st_mode));
        if (hasFrontend)
        {
            auto spaFallback = [](const drogon::HttpRequestPtr &,
                                  std::function<void(const drogon::HttpResponsePtr &)> &&cb)
            {
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_TEXT_HTML);
                std::ifstream ifs("frontend/dist/index.html");
                if (ifs.good())
                {
                    std::stringstream ss;
                    ss << ifs.rdbuf();
                    resp->setBody(ss.str());
                }
                cb(resp);
            };
            app.registerHandler("/login", spaFallback, {drogon::Get});
            app.registerHandler("/register", spaFallback, {drogon::Get});
            app.registerHandler("/borrowed", spaFallback, {drogon::Get});
            app.registerHandler("/profile", spaFallback, {drogon::Get});
            app.registerHandler("/admin", spaFallback, {drogon::Get});
            app.registerHandler("/*", spaFallback, {drogon::Get});
        }
    }
}
