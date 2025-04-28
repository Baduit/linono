add_executable(basic_test basic.cpp)
target_link_libraries(basic_test PRIVATE html_builder)
add_test(basic_test basic_test)
