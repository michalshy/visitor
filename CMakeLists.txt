cmake_minimum_required(VERSION 3.20)

project(visitor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS
    ${CMAKE_SOURCE_DIR}/src/*.cpp
)

add_executable(visitor ${SRC_FILES})

target_include_directories(visitor PRIVATE
    ${CMAKE_SOURCE_DIR}/include)

include(FetchContent)

FetchContent_Declare(
    ftxui
    GIT_REPOSITORY https://github.com/ArthurSonzogni/FTXUI.git
    GIT_TAG v5.0.0
)

FetchContent_MakeAvailable(ftxui)

target_link_libraries(visitor PRIVATE
    ftxui::screen
    ftxui::dom
    ftxui::component
)

if (MSVC)
    target_compile_options(visitor PRIVATE /W4)
else()
    target_compile_options(visitor PRIVATE -Wall -Wextra -Wpedantic)
endif()