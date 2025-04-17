import logging
from rich.logging import RichHandler
from linono_pyextractor import PyReleases

class FilterAnnoyingLog(logging.Filter):
	def filter(self, record: logging.LogRecord) -> bool:
		"""
		Filter out annoying log messages that I normally dismiss by setting
		the env variable RUST_LOG to html5ever::tree_builder=off
		"""
		return record.name != "html5ever.tree_builder"

handler = RichHandler()
handler.addFilter(FilterAnnoyingLog())
logging.basicConfig(level=logging.INFO, format="%(message)s", datefmt="[%X]", handlers=[handler])

def main():
	print("Hello from python-cli!")
	try:
		releases = PyReleases.load()
		print("Coming releases:")
		for c in releases.coming():
			print(f"\t{c.saga} {c.title} {c.release_date}")

		print("All:")		
		for saga, l in releases.all().items():
			for c in l:
				print(f"\t{c.saga} {c.title} {c.release_date}")
	except RuntimeError as e:
		print("Failed to load releases:", e)
