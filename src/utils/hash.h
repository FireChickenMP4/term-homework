#pragma once
#include <string>

std::string hashPassword(const std::string &password, const std::string &salt);
std::string randomSalt();
std::string hmacSha256(const std::string &data, const std::string &key);
