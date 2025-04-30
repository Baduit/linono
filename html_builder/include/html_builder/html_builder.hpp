#pragma once

#include <string_view>

#include <html_builder/Releases.hpp>

namespace html_builder
{

std::string releases_to_html(const Releases& releases);
std::string coming_releases_to_html(const std::vector<Release>& coming);
std::string saga_releases_to_html(std::string_view saga_name, const std::vector<Release>& saga_releases);

} // namespace html_builder
