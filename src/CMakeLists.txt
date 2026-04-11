cmake_minimum_required(VERSION 3.20)

project(visitor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(visitor_core STATIC
    core/operations.cpp
)

target_include_directories(visitor_core PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)

target_compile_options(visitor_core PRIVATE -Wall -Wextra -Wpedantic)

add_executable(visitor
    main.cpp
    visitor.cpp
    ui/pane.cpp
    ui/tui.cpp
)

include(FetchContent)

FetchContent_Declare(
    ftxui
    GIT_REPOSITORY https://github.com/ArthurSonzogni/FTXUI.git
    GIT_TAG v5.0.0
)

FetchContent_MakeAvailable(ftxui)

target_link_libraries(visitor PRIVATE
    visitor_core
    ftxui::screen
    ftxui::dom
    ftxui::component
)

target_compile_options(visitor PRIVATE -Wall -Wextra -Wpedantic)