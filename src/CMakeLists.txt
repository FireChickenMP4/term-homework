find_package(Drogon CONFIG REQUIRED)

add_executable(main main.cpp init/routes.cpp)

target_include_directories(main PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(main PRIVATE Drogon::Drogon)
