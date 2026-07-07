# Library Management API

C++20 + Drogon + MySQL 的图书馆管理系统。

## 快速开始

```bash
# 启动 MySQL
sudo service mysql start

# 构建
cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=~/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build

# 运行（首次自动建库建表，创建超级管理员 admin/admin123）
./build/src/main
```

或使用 Makefile：`make run`

## 路由

### 公开

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /health | 健康检查 |
| GET | /books | 书列表 `?page=1&limit=20` |
| GET | /books/search | 搜索 `?q=keyword`（SQL LIKE） |
| GET | /books/{id} | 书详情 |
| POST | /login | 登录 → 返回 token |
| POST | /users | 注册（固定 permission=user，自动查重） |

### 需登录（`Authorization: Bearer <token>`）

| 方法 | 路由 | 说明 |
|------|------|------|
| POST | /books | 新增书 `{"name","author","description","total"}` |
| PUT | /books/{id} | 更新书 |
| DELETE | /books/{id} | 删除书（已借出时拒绝） |
| GET | /users/{id} | 本人或管理员可看用户详情 |
| PUT | /users/{id} | 修改自身信息（需 old_password） |
| POST | /borrow | 借书 `{"user_id","book_id"}` |
| POST | /return | 还书 |
| GET | /users/{id}/books | 查看某用户的借阅列表 |

### 管理员（permission=admin）

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /users | 用户列表 |
| GET | /borrowed | 全部借书记录 |

### 仅超级管理员（id=1）

| 方法 | 路由 | 说明 |
|------|------|------|
| DELETE | /users/{id} | 删除用户（有借书时拒绝） |
| PUT | /users/{id}/permission | 提权/降权 |

## 配置

复制 `config.example.json` 为 `config.json` 修改，或使用环境变量：

```
DB_HOST=127.0.0.1  ADMIN_USER=admin
DB_USER=lib         ADMIN_PASS=admin123
DB_PASSWORD=lib
DB_NAME=lib
```

## CORS

后端默认开启 CORS（`Access-Control-Allow-Origin: *`），支持 OPTIONS 预检请求。
前端开发时可通过 API 网关或 Vite proxy 反向代理。

## 测试

```bash
cmake -B build_tests -S tests -DCMAKE_TOOLCHAIN_FILE=~/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build_tests
./build_tests/test_jwt
```
