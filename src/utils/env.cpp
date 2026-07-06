#include "env.h"
#include <cstdlib>
#include <fstream>
#include <random>
#include <sstream>

static std::string generateSecret() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 15);
    std::string hex = "0123456789abcdef";
    std::string secret;
    for (int i = 0; i < 64; i++)
        secret += hex[dis(gen)];
    return secret;
}

static std::string readEnvFile() {
    std::ifstream f(".env");
    std::string line;
    while (std::getline(f, line)) {
        if (line.find("JWT_SECRET=") == 0)
            return line.substr(11);
    }
    return "";
}

static void writeEnvFile(const std::string& secret) {
    std::ofstream f(".env", std::ios::app);
    // check if JWT_SECRET already in file
    std::ifstream in(".env");
    std::string line;
    while (std::getline(in, line))
        if (line.find("JWT_SECRET=") == 0) return;

    f << "\nJWT_SECRET=" << secret << std::endl;
}

std::string getJwtSecret() {
    auto env = std::getenv("JWT_SECRET");
    if (env && env[0]) return env;

    auto file = readEnvFile();
    if (!file.empty()) return file;

    auto secret = generateSecret();
    writeEnvFile(secret);
    return secret;
}
