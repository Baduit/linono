#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <html_builder/html_helpers.hpp>
#include <html_builder/Releases.hpp>
#include <html_builder/html_builder.hpp>

struct Toto
{
	int a;
	int b;
};

TEST_CASE("html helpers")
{
	Toto toto {.a = 5, .b = 7};
	{
		std::string tmp = html_builder::get_html_table_headers(toto);
		std::string_view expected = "<tr><th>A</th><th>B</th></tr>";
		REQUIRE(tmp == expected);
	}

	{
		std::string tmp = html_builder::range_to_html_table("my title", std::array{toto}, [](const Toto& t){ return t; });
		std::string_view expected = "<table><caption>my title</caption><tr><th>A</th><th>B</th></tr><tr><td>5</td><td>7</td></tr></table>";
		REQUIRE(tmp == expected);
	}
}

TEST_CASE("coming_releases_to_html")
{
	using namespace std::literals::chrono_literals;
	namespace c = std::chrono;

	html_builder::Release a {.saga = "lol", .title = "tome 1", .release_date = 06d / c::September / 1996y};
	html_builder::Release b {.saga = "lol", .title = "tome 2", .release_date = {}};

	{
		std::string tmp = html_builder::coming_releases_to_html({});
		std::string_view expected = "";
		REQUIRE(tmp == expected);
	}

	{
		std::string tmp = html_builder::coming_releases_to_html({a, b});
		std::string_view expected = "<table><tr><th>Saga</th><th>Title</th><th>Release date</th></tr><tr><td>lol</td><td>tome 1</td><td>06/09/1996</td></tr><tr><td>lol</td><td>tome 2</td><td>TBA</td></tr></table>";
		REQUIRE(tmp == expected);
	}
}

TEST_CASE("saga_releases_to_html")
{
	using namespace std::literals::chrono_literals;
	namespace c = std::chrono;

	html_builder::Release a {.saga = "lol", .title = "tome 1", .release_date = 06d / c::September / 1996y};
	html_builder::Release b {.saga = "lol", .title = "tome 2", .release_date = {}};

	{
		std::string tmp = html_builder::saga_releases_to_html("lol", {});
		std::string_view expected = "";
		REQUIRE(tmp == expected);
	}

	{
		std::string tmp = html_builder::saga_releases_to_html("lol", {a, b});
		std::string_view expected = "<table><caption>lol</caption><tr><th>Title</th><th>Release date</th></tr><tr><td>tome 1</td><td>06/09/1996</td></tr><tr><td>tome 2</td><td>TBA</td></tr></table>";
		REQUIRE(tmp == expected);
	}
}

TEST_CASE("releases_to_html")
{
	using namespace std::literals::chrono_literals;
	namespace c = std::chrono;

	html_builder::Release a {.saga = "lol", .title = "tome 1", .release_date = 06d / c::September / 1996y};
	html_builder::Release b {.saga = "lol", .title = "tome 2", .release_date = {}};
	html_builder::Releases releases {.all = {{"lol", {a, b}}}, .coming = {b}};

	std::string tmp = html_builder::releases_to_html(releases);
	std::string coming = "<h2>Coming releases</h2><table><tr><th>Saga</th><th>Title</th><th>Release date</th></tr><tr><td>lol</td><td>tome 2</td><td>TBA</td></tr></table>";
	std::string lol_saga = "<h2>All sagas</h2><table><caption>lol</caption><tr><th>Title</th><th>Release date</th></tr><tr><td>tome 1</td><td>06/09/1996</td></tr><tr><td>tome 2</td><td>TBA</td></tr></table>";
	std::string expected = coming + lol_saga;
	REQUIRE(tmp == expected);
}