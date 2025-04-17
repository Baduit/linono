use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::result::Result::Ok;
use chrono::NaiveDate;
use chrono::prelude::*;
use anyhow::{bail, Result};
use log::info;
use scraper::{ElementRef, Html, Selector};
use scraper::selector::ToCss;


fn get_content(url: &str) -> Result<String> {
	let body: String = ureq::get(url)
		.call()?
		.body_mut()
		.read_to_string()?;
	Ok(body)
}

#[derive(Clone, Debug)]
pub struct Release
{
	pub saga: String,
	pub title: String,
	pub release_date: Option<NaiveDate>,
}

fn get_text<'a>(element: &'a ElementRef<'a>) -> Result<&'a str> {
	let t = element.text().next();
	match t {
		Some(s) => Ok(s),
		None => bail!("No text in {}", element.html()),
	}
}

fn get_first_elem<'a>(parent: &'a ElementRef<'a>, selector: &Selector) -> Result<ElementRef<'a>> {
	let child = parent.select(selector).next();
	match child {
		Some(child) => Ok(child),
		None => bail!("Selector `{}` matched nothing in {}", selector.to_css_string(), parent.html()),
	}
}

fn get_releases_from_html(saga: &str, html_content: &str, line_selector: &Selector) -> Result<Vec<Release>> {
    let document = Html::parse_document(html_content);

    let col_selector = Selector::parse("td").unwrap();
    let numero_selector = Selector::parse("th").unwrap();

	let mut releases: Vec<Release> = Vec::new();
    for line in document.select(&line_selector) {
		let volume_elem = get_first_elem(&line, &numero_selector)?;
		let volume = get_text(&volume_elem)?.trim().to_string();

		let release_date_col = line.select(&col_selector).skip(2).next();
		let release_date_col = match release_date_col {
			Some(release_date_col) => release_date_col,
			None => bail!("Release column not found in table."),
		};
		let release_date = get_text(&release_date_col)?.trim().to_string();
		info!("Volume: {}: release date: {}", volume, release_date);

		let release_date = match NaiveDate::parse_from_str(release_date.as_str(), "%B %d, %Y") {
			Ok(date) => Some(date),
			Err(_) => None,
		};
		releases.push(Release{saga: saga.to_string(), title: volume, release_date: release_date});
    }
	Ok(releases)
}


fn add_coming_releases(all_releases: &Vec<Release>, coming_releases: &mut Vec<Release>)
{
	let today = Local::now().date_naive();
	let mut add_empty_date = true;
	for release in all_releases.iter().rev() {
		match release.release_date {
			None if add_empty_date => coming_releases.push(release.clone()),
			None => (),
			Some(date) if date >= today => {
				add_empty_date = false;
				coming_releases.push(release.clone())
			},
			Some(_) => break,
		}
	}
}

pub struct Releases
{
	pub all: BTreeMap<String, Vec<Release>>, // Sorted by saga
	pub coming: Vec<Release>,
}

impl Releases {
	pub fn load() -> Result<Releases> {
		let mut res = Releases{ all: BTreeMap::new(), coming: Vec::new() };

		res.get_slime_releases()?;
		res.get_level99_releases()?;
		res.get_go_down_in_history_releases()?;

		res.coming.sort_by(|a, b| {
			match (a.release_date, b.release_date) {
				(Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
				(Some(_), None) => Ordering::Less,
				(None, Some(_)) => Ordering::Greater,
				(None, None) if a.saga != b.saga => a.saga.cmp(&b.saga),
				(None, None) => a.title.cmp(&b.title),
			}
		});

		Ok(res)
	}

	fn get_slime_releases(&mut self) -> Result<()>{
		let saga = "That Time I Got Reincarnated as a Slime - Light Novel";

		let contents: String = get_content("https://en.m.wikipedia.org/wiki/List_of_That_Time_I_Got_Reincarnated_as_a_Slime_volumes")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse("#mf-section-1 tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases, &mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_level99_releases(&mut self) -> Result<()>{
		let saga = "Villainess_Level_99 - Light Novel";

		let contents: String = get_content("https://en.wikipedia.org/wiki/Villainess_Level_99")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases, &mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_go_down_in_history_releases(&mut self) -> Result<()>{
		let saga = "I'll Become a Villainess Who Goes Down in History - Light Novel";

		let contents: String = get_content("https://en.wikipedia.org/wiki/I'll_Become_a_Villainess_Who_Goes_Down_in_History")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}
}

