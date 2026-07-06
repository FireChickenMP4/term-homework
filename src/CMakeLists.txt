find_package(Drogon CONFIG REQUIRED)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE Drogon::Drogon)
