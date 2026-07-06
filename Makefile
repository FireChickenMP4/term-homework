VCPKG = $(HOME)/vcpkg/scripts/buildsystems/vcpkg.cmake

.PHONY: build run test testjwt clean db

build:
	cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$(VCPKG)
	cmake --build build

run: build
	./build/src/main

testjwt:
	cmake -B build_tests -S tests -DCMAKE_TOOLCHAIN_FILE=$(VCPKG)
	cmake --build build_tests
	./build_tests/test_jwt

db:
	sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &

clean:
	rm -rf build build_tests
