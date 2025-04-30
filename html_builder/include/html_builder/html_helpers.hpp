#pragma once

#include <string>
#include <format>
#include <ranges>
#include <string_view>

#include <boost/pfr/core.hpp>
#include <boost/pfr/core_name.hpp>

namespace html_builder
{

struct HtmlTagScope
{
	// Copy and move idiom to have only 1 ctor
	HtmlTagScope(std::string tag, std::string& output):
		_tag(std::move(tag)),
		_output(output)
	{
		_output += std::format("<{}>", _tag);
	}

	~HtmlTagScope()
	{
		_output += std::format("</{}>", _tag);
	}

	// Tags are often really small and with sso it should not trigger any memory allocation
	// Same thing with using std::format instead of reserving to have enough size for all the chunks then appending them one by one
	// So I don't need to overengineer this to look smarter than I am, I just need to write this comment
	std::string _tag;
	std::string& _output;
};


template <typename T>
std::string get_html_table_headers(const T& t)
{
	std::string output;
	{
		HtmlTagScope tr("tr", output);
		boost::pfr::for_each_field_with_name(t,
			[&output](std::string_view name, const auto&) {
				HtmlTagScope tr("th", output);
				output += name;
		});
	}
	return output;
}

template <std::ranges::forward_range R>
std::string range_to_html_table(std::string_view title, const R& r)
{
	if (std::ranges::empty(r))
		return "";

	std::string output;
	{
		HtmlTagScope tr("table", output);

		output += std::format("<caption>{}</caption>", title);
		output += get_html_table_headers(*std::ranges::cbegin(r));

		for (const auto& element: r)
		{
			HtmlTagScope tr("tr", output);
			boost::pfr::for_each_field(element,
				[&output]<typename T>(const T& value) {
					HtmlTagScope tr("td", output);
					output += std::format("{}", value);
			});
		}
	}
	return output;
}	

} // namespace html_builder
