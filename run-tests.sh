#!/bin/bash
set -e
cmake -B build_tests -S tests -DCMAKE_TOOLCHAIN_FILE=~/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build_tests
./build_tests/test_jwt
