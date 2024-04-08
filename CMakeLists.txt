cmake_minimum_required(VERSION 3.27)
project(orm-design)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK4 REQUIRED gtk4)

include_directories(${GTK4_INCLUDE_DIRS})
link_directories(${GTK4_LIBRARY_DIRS})

add_executable(orm-design src/main.cpp)

target_include_directories(orm-design PRIVATE ${GTK4_LIBRARY_DIRS})
target_link_libraries(orm-design ${GTK4_LIBRARIES})