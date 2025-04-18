import logging

from rich.console import Console
from rich.logging import RichHandler
from rich.table import Table

from linono_pyextractor import PyReleases

class FilterAnnoyingLog(logging.Filter):
	def filter(self, record: logging.LogRecord) -> bool:
		"""
		Filter out annoying log messages that I normally dismiss by setting
		the env variable RUST_LOG to html5ever::tree_builder=off
		"""
		return record.name != "html5ever.tree_builder"

def _all_releases_as_tables(releases: dict[str, list[PyReleases]]) -> list[Table]:
	tables = []

	for saga, release_list in releases.items():
		table = Table(title=saga)
		table.add_column("Title", justify="left")
		table.add_column("Release Date", justify="left")
		for release in release_list:
			table.add_row(release.title, str(release.release_date) if release.release_date else "N/A")
		tables.append(table)
	return tables

def main():
	handler = RichHandler()
	handler.addFilter(FilterAnnoyingLog())
	logging.basicConfig(level=logging.WARNING, format="%(message)s", datefmt="[%X]", handlers=[handler])

	releases = PyReleases.load()

	console = Console()
	for table in _all_releases_as_tables(releases.all()):
		console.print(table)
	
	table = Table(title="Coming Releases")
	table.add_column("Saga", justify="left")
	table.add_column("Title", justify="left")
	table.add_column("Release Date", justify="left")
	for release in releases.coming():
		table.add_row(release.saga, release.title, str(release.release_date) if release.release_date else "N/A")
	console.print(table)
