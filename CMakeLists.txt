cmake_minimum_required(VERSION 3.21)
project(CompetitiveProgramming)

option(TARGET_PROBLEM "Which problem to build" "a")

include(FetchContent)
FetchContent_Declare(
  googletest
  SOURCE_DIR "${CMAKE_SOURCE_DIR}/external/googletest"
)
FetchContent_MakeAvailable(googletest)

set(SRC_FILE "src/${TARGET_PROBLEM}.cpp")
add_executable(${TARGET_PROBLEM} ${SRC_FILE})

add_executable(test_${TARGET_PROBLEM} tests/generated_test_${TARGET_PROBLEM}.cpp)
target_link_libraries(test_${TARGET_PROBLEM} gtest_main)
enable_testing()

include(GoogleTest)
gtest_discover_tests(test_${TARGET_PROBLEM})