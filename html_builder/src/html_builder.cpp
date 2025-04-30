#include <iostream>

#include <html_builder/html_builder.hpp>
#include <html_builder/html_helpers.hpp>

namespace html_builder
{

struct ComingReleaseView
{
	static ComingReleaseView fromRelease(const Release& r)
	{
		return ComingReleaseView{.saga = r.saga, .title = r.title, .release_date = std::string("todo")};
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
		return SagaReleaseView{.title = r.title, .release_date = std::string("todo")};
	}

	std::string_view title;
	// Owning string because obtained throught conversion
	std::string release_date;
};

std::string releases_to_html(const Releases& releases)
{
	return "";
}

std::string coming_releases_to_html(const std::vector<Release>& coming)
{
	return range_to_html_table("Coming Releases", coming, &ComingReleaseView::fromRelease);
}

std::string saga_releases_to_html(std::string_view saga_name, const std::vector<Release>& saga_releases)
{
	return range_to_html_table(saga_name, saga_releases, &SagaReleaseView::fromRelease);
}

/*

Example de table
<table>
	<caption> title </caption>
	<tr>
		<th> saga </th>
		<th> title </th>
		<th> release_date </th>
	</tr>
	<tr>
		<td> lol </td>
		<td> 1 </td>
		<td> TBA </td>
</table>

*/

} // namespace html_builder
