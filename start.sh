#!/bin/bash
set -e

echo "=== Library Management API ==="

# 1. 启动 MySQL
echo "[1] 启动 MySQL..."
sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &

for i in $(seq 1 30); do
  if mysqladmin ping -u root --silent 2>/dev/null; then
    echo "  MySQL 已就绪"
    break
  fi
  sleep 1
done

# 2. 构建
echo "[2] 构建..."
cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$HOME/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build

# 3. 运行
echo "[3] 启动后端 http://localhost:8080"
./build/src/main
