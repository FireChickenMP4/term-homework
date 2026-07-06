#pragma once
#include <drogon/HttpFilter.h>
#include <json/value.h>

class AuthFilter : public drogon::HttpFilter<AuthFilter> {
public:
    void doFilter(const drogon::HttpRequestPtr& req,
                  drogon::FilterCallback&& fcb,
                  drogon::FilterChainCallback&& fccb) override;
};

class AdminFilter : public drogon::HttpFilter<AdminFilter> {
public:
    void doFilter(const drogon::HttpRequestPtr& req,
                  drogon::FilterCallback&& fcb,
                  drogon::FilterChainCallback&& fccb) override;
};

class SuperAdminFilter : public drogon::HttpFilter<SuperAdminFilter> {
public:
    void doFilter(const drogon::HttpRequestPtr& req,
                  drogon::FilterCallback&& fcb,
                  drogon::FilterChainCallback&& fccb) override;
};

Json::Value getCurrentUser(const drogon::HttpRequestPtr& req);
