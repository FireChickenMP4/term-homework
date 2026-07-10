#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
RELEASE_DIR="$ROOT_DIR/release"
VERSION="${1:-$(date +%Y%m%d-%H%M%S)}"
PACKAGE_NAME="library-system-$VERSION"
PACKAGE_DIR="$RELEASE_DIR/$PACKAGE_NAME"

echo "=== Build Release Package ==="

echo "[1/3] Building frontend..."
cd "$ROOT_DIR/frontend"
dx build --platform web --release 2>&1
rm -rf "$ROOT_DIR/frontend/dist"
cp -r "$ROOT_DIR/frontend/target/dx/library-system-web/release/web/public" "$ROOT_DIR/frontend/dist"

echo "[2/3] Building backend..."
cd "$ROOT_DIR"
cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE="$HOME/vcpkg/scripts/buildsystems/vcpkg.cmake"
cmake --build build

echo "[3/3] Packaging release..."
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR"

cp build/src/main "$PACKAGE_DIR/library-server"
mkdir -p "$PACKAGE_DIR/frontend"
cp -r frontend/dist "$PACKAGE_DIR/frontend/dist"
cp config.example.json "$PACKAGE_DIR/config.json"
cp start-db.sh "$PACKAGE_DIR/"
cp README.md "$PACKAGE_DIR/" 2>/dev/null || true
mkdir -p "$PACKAGE_DIR/uploads/tmp"

cat > "$PACKAGE_DIR/start.sh" << 'SCRIPT'
#!/bin/bash
set -e
ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "=== Library Management System ==="
echo "[1] 检查 MySQL..."
if ! mysqladmin ping -u root --silent 2>/dev/null; then
  echo "  启动 MySQL..."
  docker compose up -d db --wait 2>/dev/null || sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &
  for i in $(seq 1 30); do
    mysqladmin ping -u root --silent 2>/dev/null && echo "  MySQL 已就绪" && break
    sleep 1
  done
fi
echo "[2] 启动服务..."
echo "  访问 http://localhost:8808"
cd "$ROOT_DIR"
exec ./library-server
SCRIPT
chmod +x "$PACKAGE_DIR/start.sh"

echo "  Package: $PACKAGE_DIR"
echo "  Binary : library-server"
echo "  Frontend: frontend/dist/"
echo "  Config : config.json"
echo "  Scripts: start.sh, start-db.sh"

(cd "$RELEASE_DIR" && tar czf "$PACKAGE_NAME.tar.gz" "$PACKAGE_NAME")
echo "  Archive: $RELEASE_DIR/$PACKAGE_NAME.tar.gz"

echo ""
echo "=== Done ==="
