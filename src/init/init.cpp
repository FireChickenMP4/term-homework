#include "init.h"
#include "utils/config.h"
#include "utils/hash.h"
#include <mysql/mysql.h>
#include <fstream>
#include <sys/stat.h>

static bool exec(MYSQL *conn, const std::string &sql)
{
    return mysql_query(conn, sql.c_str()) == 0;
}

void init(drogon::HttpAppFramework &app)
{
    auto &cfg = AppConfig::get();

    // 连 MySQL（不指定库），建库建表 + 超级管理员
    auto *conn = mysql_init(nullptr);
    if (mysql_real_connect(conn, cfg.dbHost.c_str(), cfg.dbUser.c_str(),
                           cfg.dbPassword.c_str(), nullptr, cfg.dbPort,
                           nullptr, 0))
    {
        exec(conn, "CREATE DATABASE IF NOT EXISTS " + cfg.dbName +
                       " CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
        exec(conn, "USE " + cfg.dbName);

        exec(conn, "CREATE TABLE IF NOT EXISTS books ("
                   "id INT AUTO_INCREMENT PRIMARY KEY,"
                   "name TEXT NOT NULL, author TEXT NOT NULL,"
                   "description TEXT,"
                   "total INT NOT NULL DEFAULT 1,"
                   "available INT NOT NULL DEFAULT 1)");
        exec(conn, "CREATE TABLE IF NOT EXISTS users ("
                   "id INT AUTO_INCREMENT PRIMARY KEY,"
                   "username VARCHAR(100) UNIQUE NOT NULL,"
                   "password VARCHAR(100) NOT NULL,"
                   "permission VARCHAR(10) DEFAULT 'user',"
                   "salt VARCHAR(64) NOT NULL DEFAULT '')");
        exec(conn, "CREATE TABLE IF NOT EXISTS borrowed_books ("
                   "user_id INT NOT NULL, book_id INT NOT NULL,"
                   "PRIMARY KEY (user_id, book_id),"
                   "FOREIGN KEY (user_id) REFERENCES users(id),"
                   "FOREIGN KEY (book_id) REFERENCES books(id))");

        auto res = mysql_query(conn, "SELECT id FROM users WHERE id = 1");
        if (res == 0)
        {
            auto result = mysql_store_result(conn);
            if (result && mysql_num_rows(result) == 0)
            {
                mysql_free_result(result);
                auto salt = randomSalt();
                auto sql = "INSERT INTO users (id, username, password, permission, salt) "
                           "VALUES (1, '" +
                           cfg.adminUsername + "', '" +
                           hashPassword(cfg.adminPassword, salt) + "', 'admin', '" + salt + "')";
                exec(conn, sql);
                LOG_INFO << "Super admin created (" << cfg.adminUsername << "/"
                         << cfg.adminPassword << ")";
            }
            else if (result)
            {
                mysql_free_result(result);
            }
        }
        mysql_close(conn);
    }

    // CORS
    app.registerPostHandlingAdvice([](const drogon::HttpRequestPtr &,
                                      const drogon::HttpResponsePtr &resp)
                                   {
        resp->addHeader("Access-Control-Allow-Origin", "*");
        resp->addHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
        resp->addHeader("Access-Control-Allow-Headers", "Content-Type, Authorization"); });

    // Serve frontend static files (if built) — SPA fallback via 404 handler
    struct stat st;
    if (stat("frontend/dist", &st) == 0 && S_ISDIR(st.st_mode))
    {
        auto dist = std::filesystem::absolute("frontend/dist").string();
        app.addALocation("/", "", dist, false, true, true);
        app.setFileTypes({"html", "js", "css", "wasm", "ico", "png", "jpg", "svg", "woff2"});
        std::ifstream ifs(dist + "/index.html");
        if (ifs.good())
        {
            std::stringstream ss;
            ss << ifs.rdbuf();
            auto idx = drogon::HttpResponse::newHttpResponse();
            idx->setStatusCode(drogon::k200OK);
            idx->setContentTypeCode(drogon::CT_TEXT_HTML);
            idx->setBody(ss.str());
            app.setCustom404Page(idx, false);
        }
        LOG_INFO << "Serving frontend from " << dist;
    }
    else
    {
        LOG_WARN << "frontend/dist/ not found — frontend not built yet";
    }

    // 加载 Drogon 配置（监听端口、DB连接池、日志等）
    std::ifstream f("config.json");
    if (f.good())
    {
        f.close();
        app.loadConfigFile("config.json");
    }
    else
    {
        app.addDbClient(drogon::orm::MysqlConfig{
            .host = cfg.dbHost, .port = (unsigned short)cfg.dbPort, .databaseName = cfg.dbName, .username = cfg.dbUser, .password = cfg.dbPassword, .connectionNumber = 1, .name = "default", .isFast = false, .characterSet = "utf8mb4", .timeout = 1.0});
        app.addListener("0.0.0.0", 8808);
        app.setLogLevel(trantor::Logger::kInfo);
        Plugins::registerPlugins(app);
    }

    Routes::registerRoutes(app);
}
