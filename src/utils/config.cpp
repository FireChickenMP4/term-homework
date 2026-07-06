#include "config.h"
#include <cstdlib>

static std::string env(const char* key, const std::string& def) {
    auto v = std::getenv(key);
    return v ? v : def;
}

AppConfig& AppConfig::get() {
    static AppConfig c;
    c.dbHost = env("DB_HOST", "127.0.0.1");
    c.dbPort = std::stoi(env("DB_PORT", "3306"));
    c.dbUser = env("DB_USER", "lib");
    c.dbPassword = env("DB_PASSWORD", "lib");
    c.dbName = env("DB_NAME", "lib");
    c.adminUsername = env("ADMIN_USER", "admin");
    c.adminPassword = env("ADMIN_PASS", "admin123");
    return c;
}
