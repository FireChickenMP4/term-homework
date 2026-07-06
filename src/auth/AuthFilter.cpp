#include "AuthFilter.h"
#include "jwt.h"

void AuthFilter::doFilter(const drogon::HttpRequestPtr& req,
                          drogon::FilterCallback&& fcb,
                          drogon::FilterChainCallback&& fccb) {
    LOG_INFO << "AuthFilter called";
    auto auth = req->getHeader("Authorization");
    if (auth.size() > 7 && auth.substr(0, 7) == "Bearer ") {
        auto payload = jwt::verify(auth.substr(7));
        if (!payload.isNull()) {
            req->getAttributes()->insert("jwt", payload);
            fccb();
            return;
        }
    }
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k401Unauthorized);
    fcb(resp);
}

void AdminFilter::doFilter(const drogon::HttpRequestPtr& req,
                           drogon::FilterCallback&& fcb,
                           drogon::FilterChainCallback&& fccb) {
    auto auth = req->getHeader("Authorization");
    if (auth.size() > 7 && auth.substr(0, 7) == "Bearer ") {
        auto payload = jwt::verify(auth.substr(7));
        if (!payload.isNull() && payload["permission"].asString() == "admin") {
            req->getAttributes()->insert("jwt", payload);
            fccb();
            return;
        }
    }
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k403Forbidden);
    fcb(resp);
}

void SuperAdminFilter::doFilter(const drogon::HttpRequestPtr& req,
                                drogon::FilterCallback&& fcb,
                                drogon::FilterChainCallback&& fccb) {
    auto auth = req->getHeader("Authorization");
    if (auth.size() > 7 && auth.substr(0, 7) == "Bearer ") {
        auto payload = jwt::verify(auth.substr(7));
        if (!payload.isNull() && payload["permission"].asString() == "admin"
            && payload["user_id"].asInt() == 1) {
            req->getAttributes()->insert("jwt", payload);
            fccb();
            return;
        }
    }
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k403Forbidden);
    fcb(resp);
}

Json::Value getCurrentUser(const drogon::HttpRequestPtr& req) {
    return req->getAttributes()->get<Json::Value>("jwt");
}
