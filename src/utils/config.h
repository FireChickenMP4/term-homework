#pragma once
#include <string>

struct AppConfig {
    std::string dbHost;
    int dbPort;
    std::string dbUser;
    std::string dbPassword;
    std::string dbName;
    std::string adminUsername;
    std::string adminPassword;

    static AppConfig& get();
};
