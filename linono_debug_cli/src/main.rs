use anyhow::Result;
use linono_extractor::Releases;


fn main() -> Result<()> {
	colog::init();

	let releases = Releases::load()?;

	for release in releases.coming {
		println!("{:?}", release);
	}

	Ok(())
}
