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

set(SRC_FILE "${CMAKE_SOURCE_DIR}/src/${TARGET_PROBLEM}.cpp")
set(OUTPUT "${CMAKE_SOURCE_DIR}/generated/${TARGET_PROBLEM}_expanded.cpp")
set(ACL_INCLUDE "${CMAKE_SOURCE_DIR}/external/ac-library")
set(EXPANDER "${CMAKE_SOURCE_DIR}/tools/expander/target/release/expander")

add_custom_command(
  OUTPUT ${OUTPUT}
  COMMAND ${EXPANDER}
    --input ${SRC_FILE}
    --output ${OUTPUT}
    --include ${ACL_INCLUDE}
    --project-root ${CMAKE_SOURCE_DIR}
  DEPENDS ${SRC_FILE} ${EXPANDER}
  COMMENT "Expanding ${TARGET_PROBLEM}.cpp to ${TARGET_PROBLEM}_expanded.cpp"
)

add_custom_target(generate_expanded_src_target
  DEPENDS ${OUTPUT}
)
add_dependencies(generate_expanded_src_target expander_build)

add_custom_target(generate_expanded_src ALL
  DEPENDS generate_expanded_src_target
)

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
