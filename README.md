# 图书馆管理系统

C++20 + Drogon + MySQL 后端，Rust + Dioxus 0.7 + Tailwind CSS 前端。

## 快速开始

```bash
# 一键启动（MySQL + 构建 + 运行）
./start.sh

# 或分开执行
sudo service mysql start                 # 启动 MySQL
make                                     # 构建前后端
./build/src/main                         # 启动（http://localhost:8808）
```

首次会自动安装 `dioxus-cli` 并编译前端。之后跳过前端构建（删掉 `frontend/dist` 触发重建）。

## 技术栈

| 层 | 技术 |
|---|------|
| 后端 | C++20, Drogon, MySQL, OpenSSL, JsonCpp |
| 前端 | Rust, Dioxus 0.7, WASM, Tailwind CSS v4 |
| 构建 | CMake + vcpkg, Cargo, dioxus-cli |

## API 路由

### 公开

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /health | 健康检查 |
| GET | /books | 书列表 `?page=1&limit=9` |
| GET | /books/search | 搜索 `?q=keyword`（模糊匹配书名/作者/描述） |
| GET | /books/{id} | 书详情 |
| POST | /login | 登录 `{username, password}` → 返回 token |
| POST | /register | 注册 `{username, password}` |

### 需登录

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /me | 当前用户信息 + 借阅列表 |
| POST | /books | 新增书 `{name, author, description, total}` |
| PUT | /books/{id} | 更新书 |
| DELETE | /books/{id} | 删除书（借出时拒绝） |
| GET | /users/{id} | 用户详情 |
| PUT | /users/{id} | 修改用户名/密码（需 old_password） |
| POST | /borrow | 借书 `{user_id, book_id}` |
| POST | /return | 还书 |
| GET | /users/{id}/books | 用户借阅列表 |

### 管理员

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /users | 用户列表 |
| GET | /admin/borrowed | 全部借书记录 |
| PUT | /users/{id}/permission | 提权/降权（初始管理员可操作管理员） |
| DELETE | /users/{id} | 删除用户（管理员可删普通用户） |

## 配置

复制 `config.example.json` 为 `config.json`，或使用环境变量：

```
DB_HOST=127.0.0.1  ADMIN_USER=admin
DB_USER=lib         ADMIN_PASS=admin123
DB_PASSWORD=lib
DB_NAME=lib
```

## 测试

```bash
cmake -B build_tests -S tests -DCMAKE_TOOLCHAIN_FILE=~/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build_tests
./build_tests/test_jwt
```
