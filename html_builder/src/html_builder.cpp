#include <iostream>

#include <html_builder/html_builder.hpp>
#include <html_builder/html_helpers.hpp>

namespace html_builder
{

std::string release_date_to_string(const std::optional<std::chrono::year_month_day>& release_date)
{
	if (release_date.has_value())
		return std::format("{:%d/%m/%Y}", *release_date);
	else
		return "TBA"; // To be announced
}


struct ComingReleaseView
{
	static ComingReleaseView fromRelease(const Release& r)
	{
		return ComingReleaseView{.saga = r.saga, .title = r.title, .release_date = release_date_to_string(r.release_date)};
	}

	std::string_view saga;
	std::string_view title;
	// Owning string because obtained throught conversion
	std::string release_date;
};

struct SagaReleaseView
{
	static SagaReleaseView fromRelease(const Release& r)
	{
		return SagaReleaseView{.title = r.title, .release_date = release_date_to_string(r.release_date)};
	}

	std::string_view title;
	// Owning string because obtained throught conversion
	std::string release_date;
};

std::string releases_to_html(const Releases& releases)
{
	std::string output;

	output += "<h2>Coming releases</h2>";
	output += coming_releases_to_html(releases.coming);

	output += "<h2>All sagas</h2>";
	for (const auto& [saga, release]: releases.all)
	{
		output += saga_releases_to_html(saga, release);
	}

	return output;
}

std::string coming_releases_to_html(const std::vector<Release>& coming)
{
	return range_to_html_table("", coming, &ComingReleaseView::fromRelease);
}

std::string saga_releases_to_html(std::string_view saga_name, const std::vector<Release>& saga_releases)
{
	return range_to_html_table(saga_name, saga_releases, &SagaReleaseView::fromRelease);
}

} // namespace html_builder
