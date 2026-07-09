#pragma once
#include <string>
#include <json/value.h>

namespace jwt
{
    std::string create(int userId, const std::string &permission);
    Json::Value verify(const std::string &token);
}
