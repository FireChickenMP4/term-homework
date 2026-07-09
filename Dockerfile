# ============================================================
# Stage 1 — Build backend (C++ / Drogon via vcpkg)
# ============================================================
FROM ubuntu:22.04 AS backend-builder

RUN apt-get update && apt-get install -y \
    build-essential cmake git curl zip unzip tar pkg-config \
    libssl-dev libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

ENV VCPKG_ROOT=/opt/vcpkg
ENV PATH=${VCPKG_ROOT}:${PATH}

RUN git clone https://github.com/microsoft/vcpkg ${VCPKG_ROOT} \
    && ${VCPKG_ROOT}/bootstrap-vcpkg.sh

RUN ${VCPKG_ROOT}/vcpkg install drogon

WORKDIR /app
COPY CMakeLists.txt .
COPY src/ src/
RUN cmake -B build -S . \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_TOOLCHAIN_FILE=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake \
    && cmake --build build -j$(nproc) \
    && cp build/src/main /library-server

# ============================================================
# Stage 2 — Build frontend (Rust / Dioxus WASM)
# ============================================================
FROM rust:slim-bookworm AS frontend-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install dioxus-cli

WORKDIR /app
COPY frontend/ frontend/

RUN sed -i 's|http://localhost:8808||' frontend/src/api.rs
RUN cd frontend && dx build --platform web --release
RUN cp -r frontend/target/dx/library-system-web/release/web/public /frontend-dist

# ============================================================
# Stage 3 — Runtime image
# ============================================================
FROM ubuntu:22.04 AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /library-server /app/
COPY --from=backend-builder /opt/vcpkg/installed/x64-linux/lib/libdrogon* \
    /opt/vcpkg/installed/x64-linux/lib/libtrantor* \
    /app/lib/
COPY --from=frontend-builder /frontend-dist /app/frontend/dist
COPY docker-entrypoint.sh /app/
COPY config.docker.json /app/config.json

ENV LD_LIBRARY_PATH=/app/lib

WORKDIR /app
RUN mkdir -p uploads/tmp

EXPOSE 8808

ENTRYPOINT ["/app/docker-entrypoint.sh"]
