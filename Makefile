VCPKG = $(HOME)/vcpkg/scripts/buildsystems/vcpkg.cmake
CARGO = $(HOME)/.cargo/bin/cargo
DX = $(HOME)/.cargo/bin/dx
PROXY = http://172.29.0.1:18800

.PHONY: build frontend run test testjwt db clean

build: frontend
	cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$(VCPKG)
	cmake --build build

frontend:
	cd frontend && http_proxy=$(PROXY) https_proxy=$(PROXY) $(DX) build --platform web --release 2>&1 | grep -E "(Compiled|Client build)" | tail -2
	rm -rf frontend/dist
	cp -r frontend/target/dx/library-system-web/release/web/public frontend/dist

run: build
	./build/src/main

test: testjwt

testjwt:
	cmake -B build_tests -S tests -DCMAKE_TOOLCHAIN_FILE=$(VCPKG)
	cmake --build build_tests
	./build_tests/test_jwt

db:
	docker compose up -d db --wait 2>/dev/null || sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &

clean:
	rm -rf build build_tests frontend/dist
