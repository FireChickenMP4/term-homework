#include "base64.h"

static const std::string B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

std::string b64urlEncode(const std::string &in)
{
    std::string out;
    out.reserve(((in.size() + 2) / 3) * 4);
    for (size_t i = 0; i < in.size(); i += 3)
    {
        int b = (unsigned char)in[i] << 16;
        if (i + 1 < in.size())
            b |= (unsigned char)in[i + 1] << 8;
        if (i + 2 < in.size())
            b |= (unsigned char)in[i + 2];
        out += B64[(b >> 18) & 0x3f];
        out += B64[(b >> 12) & 0x3f];
        out += B64[(b >> 6) & 0x3f];
        out += B64[b & 0x3f];
    }
    auto pad = out.size() % 4;
    if (pad)
        out.resize(out.size() - pad);
    for (auto &c : out)
    {
        if (c == '+')
            c = '-';
        else if (c == '/')
            c = '_';
    }
    return out;
}

std::string b64urlDecode(const std::string &in)
{
    std::string s = in;
    s.append((4 - s.size() % 4) % 4, '=');
    for (auto &c : s)
    {
        if (c == '-')
            c = '+';
        else if (c == '_')
            c = '/';
    }

    std::string out;
    for (size_t i = 0; i < s.size(); i += 4)
    {
        if (s[i] == '=')
            break;
        int b = B64.find(s[i]) << 18;
        if (s[i + 1] != '=')
            b |= B64.find(s[i + 1]) << 12;
        if (s[i + 2] != '=')
            b |= B64.find(s[i + 2]) << 6;
        if (s[i + 3] != '=')
            b |= B64.find(s[i + 3]);
        out += (b >> 16) & 0xff;
        if (s[i + 2] != '=')
            out += (b >> 8) & 0xff;
        if (s[i + 3] != '=')
            out += b & 0xff;
    }
    return out;
}
