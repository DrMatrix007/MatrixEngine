cmake_minimum_required(VERSION 3.0.0)
project(MatrixEngine VERSION 0.1.0)

include(CTest)
enable_testing()

find_package(SDL2 REQUIRED)

file(GLOB_RECURSE SOURCES src/**.cpp src/**.h)

add_executable(MatrixEngine main.cpp ${SOURCES})

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})

target_link_libraries(MatrixEngine ${SDL2_LIBRARIES})

include(CPack)

