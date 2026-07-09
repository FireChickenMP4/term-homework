# ============================================================
# Stage 1 — Build Drogon framework from source
# ============================================================
FROM ubuntu:22.04 AS drogon-builder

RUN apt-get update && apt-get install -y \
    build-essential cmake git pkg-config \
    libssl-dev libjsoncpp-dev zlib1g-dev uuid-dev libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

RUN git clone --depth 1 --recurse-submodules https://github.com/drogonframework/drogon /tmp/drogon

WORKDIR /tmp/drogon/build
RUN cmake .. -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTING=OFF -DBUILD_EXAMPLES=OFF
RUN cmake --build . -j$(nproc)
RUN cmake --install .

WORKDIR /app

# ============================================================
# Stage 2 — Build backend (C++ / Drogon)
# ============================================================
FROM ubuntu:22.04 AS backend-builder

COPY --from=drogon-builder /usr/local /usr/local

RUN apt-get update && apt-get install -y \
    build-essential cmake pkg-config \
    libssl-dev libjsoncpp-dev zlib1g-dev uuid-dev libmysqlclient-dev \
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
FROM rust:slim-bookworm AS frontend-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install dioxus-cli --locked

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
    ca-certificates libjsoncpp-dev libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /library-server /app/
COPY --from=drogon-builder /usr/local/lib /usr/local/lib/
COPY --from=frontend-builder /frontend-dist /app/frontend/dist
COPY docker-entrypoint.sh /app/
COPY config.docker.json /app/config.json

RUN ldconfig

WORKDIR /app
RUN mkdir -p uploads/tmp

EXPOSE 8808

ENTRYPOINT ["/app/docker-entrypoint.sh"]
