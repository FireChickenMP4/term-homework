cmake_minimum_required(VERSION 3.20)
project(test_lib LANGUAGES CXX)

find_package(OpenSSL REQUIRED)
find_package(jsoncpp CONFIG REQUIRED)
find_package(GTest REQUIRED)

add_executable(test_jwt
    ../src/auth/jwt.cpp
    ../src/utils/env.cpp
    ../src/utils/hash.cpp
    ../src/utils/base64.cpp
    test_main.cpp
)

target_include_directories(test_jwt PRIVATE ../src)
target_link_libraries(test_jwt PRIVATE GTest::gtest_main
    OpenSSL::SSL OpenSSL::Crypto JsonCpp::JsonCpp)
