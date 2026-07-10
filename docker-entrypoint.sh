#!/bin/bash

# 等待 MySQL 就绪
for i in $(seq 1 30); do
  timeout 2 bash -c "echo >/dev/tcp/${DB_HOST:-db}/${DB_PORT:-3306}" 2>/dev/null && break
  echo "Waiting for MySQL... ($i/30)"
  sleep 2
done

# 从环境变量生成 config.json
cat > config.json <<EOF
{
    "listeners": [
        {
            "address": "0.0.0.0",
            "port": 8808
        }
    ],
    "db_clients": [
        {
            "name": "default",
            "rdbms": "mysql",
            "host": "${DB_HOST:-db}",
            "port": ${DB_PORT:-3306},
            "dbname": "${DB_NAME:-lib}",
            "user": "${DB_USER:-lib}",
            "password": "${DB_PASSWORD:-lib}",
            "characterSet": "utf8mb4",
            "connection_number": 1,
            "timeout": 1.0
        }
    ],
    "plugins": [
        {
            "name": "drogon::plugin::AccessLogger",
            "config": {
                "log_format": "\$remote_addr \$method \$url \$status \$body_bytes_sent \$processing_time"
            }
        }
    ],
    "max_connection_size": 100000,
    "log": {
        "log_path": "./",
        "logfile_base_name": "lib",
        "log_size_limit": 10485760,
        "log_level": "INFO"
    }
}
EOF

exec ./library-server
