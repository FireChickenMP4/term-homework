find_package(Drogon CONFIG REQUIRED)

add_executable(main main.cpp init/init.cpp init/routes.cpp models/models.cpp controller/book.cpp controller/user.cpp controller/lib.cpp auth/jwt.cpp auth/AuthFilter.cpp utils/env.cpp utils/hash.cpp utils/config.cpp utils/base64.cpp)

target_include_directories(main PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(main PRIVATE Drogon::Drogon)
