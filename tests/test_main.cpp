#include <gtest/gtest.h>
#include "auth/jwt.h"
#include "utils/env.h"
#include "utils/hash.h"

TEST(JwtTest, CreateAndVerify) {
    auto token = jwt::create(1, "admin");
    EXPECT_FALSE(token.empty());
    EXPECT_EQ(std::count(token.begin(), token.end(), '.'), 2);
}

TEST(JwtTest, ValidTokenVerified) {
    auto token = jwt::create(42, "user");
    auto payload = jwt::verify(token);
    EXPECT_FALSE(payload.isNull());
    EXPECT_EQ(payload["user_id"].asInt(), 42);
    EXPECT_EQ(payload["permission"].asString(), "user");
}

TEST(JwtTest, InvalidTokenFails) {
    auto payload = jwt::verify("invalid.token.here");
    EXPECT_TRUE(payload.isNull());
}

TEST(JwtTest, TamperedTokenFails) {
    auto token = jwt::create(1, "admin");
    token[token.size() - 1] ^= 1;
    auto payload = jwt::verify(token);
    EXPECT_TRUE(payload.isNull());
}

TEST(HashTest, SameInputSameHash) {
    auto salt = "test_salt_123";
    auto h1 = hashPassword("hello", salt);
    auto h2 = hashPassword("hello", salt);
    EXPECT_EQ(h1, h2);
}

TEST(HashTest, DifferentInputDifferentHash) {
    auto h1 = hashPassword("password1", "salt1");
    auto h2 = hashPassword("password2", "salt1");
    EXPECT_NE(h1, h2);
}

TEST(HashTest, DifferentSaltDifferentHash) {
    auto h1 = hashPassword("same", "salt_a");
    auto h2 = hashPassword("same", "salt_b");
    EXPECT_NE(h1, h2);
}

TEST(HashTest, SaltIsRandom) {
    auto s1 = randomSalt();
    auto s2 = randomSalt();
    EXPECT_NE(s1, s2);
    EXPECT_EQ(s1.size(), 32u);
    EXPECT_EQ(s2.size(), 32u);
}

TEST(EnvTest, JwtSecretNotEmpty) {
    auto secret = getJwtSecret();
    EXPECT_FALSE(secret.empty());
    EXPECT_GE(secret.size(), 32u);
}
