cmake_minimum_required(VERSION 3.21)
project(CompetitiveProgramming)

option(TARGET_PROBLEM "Which problem to build" "a")

# Fetch GoogleTest only if needed (省略可能だが残しても良い)
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/v1.14.0.zip
)
FetchContent_MakeAvailable(googletest)

set(SRC_FILE "src/${TARGET_PROBLEM}.cpp")
add_executable(${TARGET_PROBLEM} ${SRC_FILE})

add_executable(test_${TARGET_PROBLEM} tests/generated_test_${TARGET_PROBLEM}.cpp)
target_link_libraries(test_${TARGET_PROBLEM} gtest_main)
enable_testing()
# add_test(NAME ${TARGET_PROBLEM}_test COMMAND test_${TARGET_PROBLEM})

include(GoogleTest)
gtest_discover_tests(test_${TARGET_PROBLEM})