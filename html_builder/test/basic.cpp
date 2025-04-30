#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <iostream>

#include <html_builder/html_helpers.hpp>

struct Toto
{
	int a;
	int b;
};

TEST_CASE("basic")
{
	Toto toto {.a = 5, .b = 7};
	{
		std::string tmp = html_builder::get_html_table_headers(toto);
		std::string_view expected = "<tr><th>a</th><th>b</th></tr>";
		REQUIRE(tmp == expected);
	}

	{
		std::string tmp = html_builder::range_to_html_table("my title", std::array{toto});
		std::cout << tmp << std::endl;
		std::string_view expected = "<table><caption>my title</caption><tr><th>a</th><th>b</th></tr><tr><td>5</td><td>7</td></tr></table>";
		REQUIRE(tmp == expected);
	}
}
