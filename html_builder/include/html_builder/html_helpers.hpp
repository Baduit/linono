#pragma once

#include <algorithm>
#include <string>
#include <format>
#include <ranges>
#include <string_view>
#include <cctype>

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


inline std::string format_header(std::string_view header)
{
	std::string output = std::string(header);
	output.reserve(header.size());
	// Casts needed because https://en.cppreference.com/w/cpp/string/byte/toupper
	output[0] = static_cast<char>(std::toupper(static_cast<unsigned char>(output[0])));
	std::transform(output.begin(), output.end(), output.begin(),
		[](char c)
		{
			return c == '_' ? ' ' : c;
		});
	return output;
}

template <typename T>
std::string get_html_table_headers(const T& t)
{
	std::string output;
	{
		HtmlTagScope tr("tr", output);
		boost::pfr::for_each_field_with_name(t,
			[&output](std::string_view name, const auto&) {
				HtmlTagScope tr("th", output);
				output += format_header(name);
		});
	}
	return output;
}

template<typename T>
std::string add_row(const T& element)
{
	std::string output;
	HtmlTagScope tr("tr", output);
	boost::pfr::for_each_field(element,
		[&output]<typename T>(const T& value) {
			HtmlTagScope tr("td", output);
			output += std::format("{}", value);
	});
	return output;
}

template <std::ranges::forward_range R, typename F>
std::string range_to_html_table(std::string_view title, const R& r, F&& transform_function)
{
	if (std::ranges::empty(r))
		return "";

	std::string output;
	{
		HtmlTagScope tr("table", output);

		if (title != "")
			output += std::format("<caption>{}</caption>", title);

		auto first_element = transform_function(*std::ranges::cbegin(r));
		output += get_html_table_headers(first_element);
		output += add_row(first_element);

		for (const auto& element: r | std::views::drop(1) | std::views::transform(transform_function))
		{
			output += add_row(element);
		}
	}
	return output;
}	

} // namespace html_builder
