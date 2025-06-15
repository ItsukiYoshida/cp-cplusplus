cmake_minimum_required(VERSION 3.21)
project(CompetitiveProgramming)

option(TARGET_PROBLEM "Which problem to build" "a")

find_package(Boost REQUIRED)

include(FetchContent)
FetchContent_Declare(
  googletest
  SOURCE_DIR "${CMAKE_SOURCE_DIR}/external/googletest"
)
FetchContent_MakeAvailable(googletest)

set(SRC_FILE "src/${TARGET_PROBLEM}.cpp")
add_executable(${TARGET_PROBLEM} ${SRC_FILE})

target_include_directories(${TARGET_PROBLEM} PRIVATE ${Boost_INCLUDE_DIRS} external/ac-library)
target_link_libraries(${TARGET_PROBLEM} ${Boost_LIBRARIES})

add_executable(test_${TARGET_PROBLEM} tests/generated_test_${TARGET_PROBLEM}.cpp)
target_include_directories(test_${TARGET_PROBLEM} PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(test_${TARGET_PROBLEM} gtest_main ${Boost_LIBRARIES})
enable_testing()

include(GoogleTest)
gtest_discover_tests(test_${TARGET_PROBLEM}
  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
  PROPERTIES
    ENVIRONMENT "GTEST_OUTPUT=xml:${CMAKE_SOURCE_DIR}/test-results/${TARGET_PROBLEM}.xml"
)
