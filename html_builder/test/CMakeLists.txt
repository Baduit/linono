find_package(doctest REQUIRED)

add_executable(basic_test basic.cpp)
target_link_libraries(basic_test PRIVATE html_builder doctest::doctest)
add_test(basic_test basic_test)
