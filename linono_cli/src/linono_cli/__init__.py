import logging
from linono_pyextractor import PyReleases

class FilterAnnoyingLog(logging.Filter):
	def filter(self, record: logging.LogRecord) -> bool:
		"""
		Filter out annoying log messages that I normally dismiss by setting
		the env variable RUST_LOG to html5ever::tree_builder=off
		"""
		return record.name != "html5ever.tree_builder"

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'

handler = logging.StreamHandler()
handler.setFormatter(logging.Formatter(FORMAT))
handler.addFilter(FilterAnnoyingLog())

logger = logging.getLogger()
logger.addHandler(handler)
logger.setLevel(logging.INFO)

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
