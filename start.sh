#!/bin/bash
set -e

echo "=== Library Management System ==="
ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
export PATH="$HOME/.cargo/bin:$PATH"
export http_proxy=http://172.29.0.1:18800
export https_proxy=http://172.29.0.1:18800

# 1. 检查 MySQL
if mysqladmin ping -u root --silent 2>/dev/null; then
  echo "[1] MySQL 已在运行"
else
  echo "[1] 启动 MySQL..."
  sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &
  for i in $(seq 1 30); do
    if mysqladmin ping -u root --silent 2>/dev/null; then
      echo "  MySQL 已就绪"
      break
    fi
    sleep 1
  done
fi

# 2. 构建前端（检测源码变更）
FRONTEND_OUT="$ROOT_DIR/frontend/dist/index.html"
if [ -f "$FRONTEND_OUT" ] && [ -z "$(find "$ROOT_DIR/frontend/src" -newer "$FRONTEND_OUT" -name '*.rs' 2>/dev/null | head -1)" ]; then
  echo "[2] 前端无变动，跳过"
else
  echo "[2] 构建前端..."
  cd "$ROOT_DIR/frontend"
  dx build --platform web --release --verbose 2>&1 | grep -E "(error|warning:|Compiled|Client build)"
  cp -r "$ROOT_DIR/frontend/target/dx/library-system-web/release/web/public" "$ROOT_DIR/frontend/dist"
fi

# 3. 构建后端（检测源码变更）
BACKEND_OUT="$ROOT_DIR/build/src/main"
if [ -f "$BACKEND_OUT" ] && [ -z "$(find "$ROOT_DIR/src" -newer "$BACKEND_OUT" \( -name '*.cpp' -o -name '*.h' \) 2>/dev/null | head -1)" ]; then
  echo "[3] 后端无变动，跳过"
else
  echo "[3] 构建后端..."
  cd "$ROOT_DIR"
  cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$HOME/vcpkg/scripts/buildsystems/vcpkg.cmake
  cmake --build build
fi

# 4. 启动
echo ""
echo "  访问 http://localhost:8808"
echo ""
cd "$ROOT_DIR"
./build/src/main
