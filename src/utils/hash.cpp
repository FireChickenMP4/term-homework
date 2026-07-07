#include "hash.h"
#include <openssl/sha.h>
#include <openssl/hmac.h>
#include <random>
#include <sstream>
#include <iomanip>

std::string hashPassword(const std::string &password, const std::string &salt)
{
    auto input = salt + password;
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256((unsigned char *)input.data(), input.size(), hash);
    std::ostringstream out;
    for (auto c : hash)
        out << std::hex << std::setw(2) << std::setfill('0') << (int)c;
    return out.str();
}

std::string randomSalt()
{
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 15);
    const char hex[] = "0123456789abcdef";
    std::string salt;
    for (int i = 0; i < 32; i++)
        salt += hex[dis(gen)];
    return salt;
}

std::string hmacSha256(const std::string &data, const std::string &key)
{
    unsigned char digest[32];
    unsigned int len = 32;
    HMAC(EVP_sha256(), key.data(), (int)key.size(),
         (const unsigned char *)data.data(), data.size(), digest, &len);
    return std::string((char *)digest, 32);
}
