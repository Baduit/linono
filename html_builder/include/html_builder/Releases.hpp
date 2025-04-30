#pragma once

#include <string>
#include <optional>
#include <map>
#include <vector>
#include <chrono>

namespace html_builder
{

struct Release
{
	std::string saga;
	std::string title;
	std::optional<std::chrono::year_month_day> release_date;
};

struct Releases
{
	std::map<std::string, std::vector<Release>> all;
	std::vector<Release> coming;
};	

} // namespace html_builder
