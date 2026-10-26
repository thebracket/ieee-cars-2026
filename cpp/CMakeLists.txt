cmake_minimum_required(VERSION 3.21)

project(ieee_cars_cpp_demos LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Threads REQUIRED)

# These demos rely on unoptimized code to keep the races observable.
# Default to Debug when no build type was supplied (single-config generators).
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

function(add_cpp_demo name)
  add_executable(${name} ${name}/main.cpp)
  target_link_libraries(${name} PRIVATE Threads::Threads)

  if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
    target_compile_options(${name} PRIVATE -Wall -Wextra)
  endif()
endfunction()

add_cpp_demo(hello_world)
add_cpp_demo(move_by_default)
add_cpp_demo(borrowing)
add_cpp_demo(data_race)
add_cpp_demo(cpp_atomic)
add_cpp_demo(cpp_mutex)
