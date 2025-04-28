find_package(Boost REQUIRED)

add_library(html_builder src/main.cpp)
target_include_directories(html_builder PUBLIC include PRIVATE src)
target_link_libraries(html_builder Boost::boost)

include(CTest)
enable_testing(MAKE_DIRECTORY test)
add_subdirectory(test)
