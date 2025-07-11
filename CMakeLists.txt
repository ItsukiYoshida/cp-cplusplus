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

add_custom_command(
  OUTPUT ${CMAKE_SOURCE_DIR}/tools/expander/target/release/expander
  COMMAND cargo build --release
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/tools/expander
  COMMENT "Building Rust expander"
)

add_custom_target(expander_build
  DEPENDS ${CMAKE_SOURCE_DIR}/tools/expander/target/release/expander
)

if(NOT DEFINED USER_SOURCE_DIR)
  set(USER_SOURCE_DIR "${CMAKE_SOURCE_DIR}")
endif()

set(SRC_FILE "${USER_SOURCE_DIR}/src/${TARGET_PROBLEM}.cpp")
set(ACL_INCLUDE_DIRS "${CMAKE_SOURCE_DIR}/external/ac-library")

add_executable(${TARGET_PROBLEM} ${SRC_FILE})
target_compile_definitions(${TARGET_PROBLEM} PRIVATE LOCAL)
include_directories(include)

target_include_directories(${TARGET_PROBLEM} PRIVATE ${Boost_INCLUDE_DIRS} ${ACL_INCLUDE_DIRS})
target_link_libraries(${TARGET_PROBLEM} ${Boost_LIBRARIES})

add_executable(test_${TARGET_PROBLEM} ${USER_SOURCE_DIR}/tests/generated_test_${TARGET_PROBLEM}.cpp)
target_include_directories(test_${TARGET_PROBLEM} PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(test_${TARGET_PROBLEM} gtest_main ${Boost_LIBRARIES})
enable_testing()

include(GoogleTest)
gtest_discover_tests(test_${TARGET_PROBLEM}
  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
  PROPERTIES
)
