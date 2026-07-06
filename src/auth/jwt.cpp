#include "jwt.h"
#include "utils/env.h"
#include "utils/base64.h"
#include "utils/hash.h"
#include <json/writer.h>
#include <json/reader.h>

static const std::string& SECRET() {
    static std::string s = getJwtSecret();
    return s;
}

namespace jwt {

std::string create(int userId, const std::string& permission) {
    Json::StreamWriterBuilder writer;
    Json::Value header; header["alg"] = "HS256"; header["typ"] = "JWT";
    Json::Value payload; payload["user_id"] = userId;
    payload["permission"] = permission;
    payload["exp"] = (Json::Value::Int64)(time(nullptr) + 86400 * 7);

    auto b64h = b64urlEncode(Json::writeString(writer, header));
    auto b64p = b64urlEncode(Json::writeString(writer, payload));
    auto sig = b64urlEncode(hmacSha256(b64h + "." + b64p, SECRET()));
    return b64h + "." + b64p + "." + sig;
}

Json::Value verify(const std::string& token) {
    auto dot1 = token.find('.');
    auto dot2 = token.rfind('.');
    if (dot1 == std::string::npos || dot2 == std::string::npos) return {};

    auto b64h = token.substr(0, dot1);
    auto b64p = token.substr(dot1 + 1, dot2 - dot1 - 1);
    auto sig = token.substr(dot2 + 1);

    if (b64urlEncode(hmacSha256(b64h + "." + b64p, SECRET())) != sig) return {};

    auto jsonStr = b64urlDecode(b64p);
    Json::Value payload;
    auto* reader = Json::CharReaderBuilder().newCharReader();
    std::string err;
    reader->parse(jsonStr.data(), jsonStr.data() + jsonStr.size(), &payload, &err);
    delete reader;

    if (payload["exp"].asInt64() < time(nullptr)) return {};
    return payload;
}

} // namespace jwt
