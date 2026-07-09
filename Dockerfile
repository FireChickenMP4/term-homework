# ============================================================
# Stage 1 — Build Drogon framework from source (cached layer)
# ============================================================
FROM ubuntu:22.04 AS drogon-builder

RUN apt-get update && apt-get install -y \
    build-essential cmake git \
    libssl-dev libjsoncpp-dev zlib1g-dev uuid-dev libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

RUN git clone --depth 1 --branch v1.10.0 https://github.com/drogonframework/drogon /tmp/drogon \
    && cmake -B /tmp/drogon/build -DCMAKE_BUILD_TYPE=Release \
    && cmake --build /tmp/drogon/build -j$(nproc) \
    && cmake --install /tmp/drogon/build \
    && rm -rf /tmp/drogon

# ============================================================
# Stage 2 — Build backend (C++ / Drogon)
# ============================================================
FROM ubuntu:22.04 AS backend-builder

COPY --from=drogon-builder /usr/local /usr/local

RUN apt-get update && apt-get install -y \
    build-essential cmake \
    libssl-dev libjsoncpp-dev zlib1g-dev uuid-dev libmysqlclient-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY CMakeLists.txt .
COPY src/ src/
RUN cmake -B build -S . -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build -j$(nproc) \
    && cp build/src/main /library-server

# ============================================================
# Stage 3 — Build frontend (Rust / Dioxus WASM)
# ============================================================
FROM rust:1.85-slim-bookworm AS frontend-builder

RUN cargo install dioxus-cli

WORKDIR /app
COPY frontend/ frontend/

RUN sed -i 's|http://localhost:8808||' frontend/src/api.rs
RUN cd frontend && dx build --platform web --release
RUN cp -r frontend/target/dx/library-system-web/release/web/public /frontend-dist

# ============================================================
# Stage 4 — Runtime image
# ============================================================
FROM ubuntu:22.04 AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 libjsoncpp-dev libmysqlclient21 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /library-server /app/
COPY --from=frontend-builder /frontend-dist /app/frontend/dist
COPY docker-entrypoint.sh /app/
COPY config.docker.json /app/config.json

WORKDIR /app
RUN mkdir -p uploads/tmp

EXPOSE 8808

ENTRYPOINT ["/app/docker-entrypoint.sh"]
