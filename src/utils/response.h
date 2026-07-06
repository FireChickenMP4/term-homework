#pragma once
#include <drogon/HttpResponse.h>
#include <json/value.h>

inline drogon::HttpResponsePtr err(const std::string& msg, int code = 400) {
    Json::Value v; v["error"] = msg;
    auto r = drogon::HttpResponse::newHttpJsonResponse(v);
    r->setStatusCode((drogon::HttpStatusCode)code);
    return r;
}
