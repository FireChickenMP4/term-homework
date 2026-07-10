# 图书馆管理系统

C++20 + Drogon + MySQL 后端，Rust + Dioxus 0.7 + Tailwind CSS 前端。  
Docker Compose 一键部署，GitHub Actions 自动构建 & 发布。

## 技术栈

| 层 | 技术 |
|---|------|
| 后端 | C++20, Drogon 1.9, MySQL 8.0, OpenSSL |
| 前端 | Rust, Dioxus 0.7, WASM, Tailwind CSS v4 |
| 网关 | Caddy 2 + ratelimit 插件 |
| CI/CD | GitHub Actions, Docker Buildx, Docker Hub |
| 证书 | ZeroSSL (via Caddy ACME) |

## 快速开始（本地开发）

```bash
# 依赖：MySQL 8.0+, vcpkg, Rust toolchain, dioxus-cli

# 启动 MySQL
sudo service mysql start

# 构建并运行
make run

# 访问 http://localhost:8808
```

## Docker 部署

```bash
# 构建并启动全部服务
docker compose up -d

# 查看状态
docker compose ps
docker compose logs app
```

服务说明：

| 服务 | 镜像 | 端口 | 说明 |
|------|------|------|------|
| app | firechickenmp4/library-system | 8808 | Drogon 后端 |
| caddy | firechickenmp4/lib-caddy | 80/443 | 反向代理 + 静态文件 |
| db | mysql:8.0 | 3306 | MySQL 数据库 |

## CI/CD

GitHub Actions 自动流程（`.github/workflows/deploy.yml`）：

1. **changes** — 检测后端/前端文件变更
2. **backend** — 构建 Docker 镜像并推送到 Docker Hub（打 SHA + latest 标签）
3. **caddy** — 构建 Caddy + 前端 WASM 镜像并推送
4. **deploy** — SSH 到服务器，拉取新镜像，重启对应容器

触发条件：

| 变更 | backend | caddy | deploy |
|------|---------|-------|--------|
| `src/**` / `Dockerfile` | ✅ 构建 | ❌ 跳过 | 重启全部 |
| `frontend/**` / `Dockerfile.caddy` | ❌ 跳过 | ✅ 构建 | 仅重启 caddy |
| `.github/workflows/deploy.yml` | ✅ 构建 | ✅ 构建 | 重启全部 |
| 其他（README 等） | ❌ 跳过 | ❌ 跳过 | 什么都不做 |

## API 路由

### 公开

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /api/health | 健康检查 |
| GET | /api/books | 书列表 `?page=1&limit=9` |
| GET | /api/books/search | 搜索 `?q=keyword` |
| GET | /api/books/{id} | 书详情 |
| POST | /api/login | 登录 `{username, password}` → token |
| POST | /api/register | 注册 `{username, password}` |

### 需登录（Authorization: Bearer \<token\>）

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /api/me | 当前用户信息 + 借阅列表 |
| POST | /api/books | 新增书 |
| PUT | /api/books/{id} | 更新书 |
| DELETE | /api/books/{id} | 删除书 |
| PUT | /api/users/{id} | 修改用户名/密码 |
| POST | /api/borrow | 借书 `{user_id, book_id}` |
| POST | /api/return | 还书 |
| GET | /api/users/{id}/books | 用户借阅列表 |

### 管理员

| 方法 | 路由 | 说明 |
|------|------|------|
| GET | /api/users | 用户列表 |
| GET | /api/admin/borrowed | 全部借书记录 |
| PUT | /api/users/{id}/permission | 提权/降权 |
| DELETE | /api/users/{id} | 删除用户 |

## 配置

支持环境变量覆盖（默认值）：

```
DB_HOST=127.0.0.1    DB_PORT=3306
DB_USER=lib          DB_PASSWORD=lib
DB_NAME=lib
ADMIN_USER=admin     ADMIN_PASS=admin123
JWT_SECRET=change-me-in-production
```

Docker 部署时可通过 `.env` 文件或 `docker compose run -e` 传入。

## 默认账号

| 角色 | 用户名 | 密码 |
|------|--------|------|
| 管理员 | admin | admin123 |

## 测试

```bash
make test
```

## 项目结构

```
├── .github/workflows/deploy.yml   # CI/CD 流水线
├── src/                            # C++ 后端
│   ├── main.cpp                    # 入口
│   ├── init/                       # 初始化、路由注册
│   ├── controller/                 # API 处理器
│   ├── models/                     # 数据模型
│   ├── auth/                       # JWT + 过滤器
│   └── utils/                      # 工具函数
├── frontend/                       # Rust WASM 前端
│   └── src/                        # 组件 + API 调用
├── tests/                          # 后端测试
├── Dockerfile                      # 后端镜像
├── Dockerfile.caddy                # 前端 + Caddy 镜像
├── docker-compose.yml              # 编排文件
└── Caddyfile                       # 反向代理配置
```
