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
		let release_date = get_text(&release_date_col)?
			.to_string()
			.replace("(e-book)", "")
			.replace("(Digital)", "");
		info!("Volume: {}: release date: {}", volume, release_date);

		let release_date = match NaiveDate::parse_from_str(release_date.as_str().trim(), "%B %d, %Y") {
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
		res.get_apothecary_diaries_releases()?;
		res.get_next_life_as_vilainess_doom_releases()?;
		res.get_bookworm_hannelor()?;
		res.get_secrets_of_the_silent_witch()?;
		res.get_executioner()?;
		res.get_magical_revolution_princess_genius()?;
		res.get_bofuri()?;

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
			&Selector::parse(".wikitable:nth-of-type(1) tr:has(>th):has(>td)").unwrap(),
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

	fn get_apothecary_diaries_releases(&mut self) -> Result<()>{
		let saga = "The Apothecary Diaries - Light Novel";

		let contents: String = get_content("https://en.wikipedia.org/wiki/List_of_The_Apothecary_Diaries_volumes")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(1) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_next_life_as_vilainess_doom_releases(&mut self) -> Result<()>{
		let saga = "My Next Life as a Villainess: All Routes Lead to Doom! - Light Novel";

		let contents: String = get_content("https://en.wikipedia.org/wiki/My_Next_Life_as_a_Villainess:_All_Routes_Lead_to_Doom!")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_bookworm_hannelor(&mut self) -> Result<()>{
		let saga = "Ascendance of a Bookworm - Hannelore's Fifth Year at the Royal Academy";

		let contents: String = get_content("https://en.wikipedia.org/wiki/List_of_Ascendance_of_a_Bookworm_light_novels")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(3) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_secrets_of_the_silent_witch(&mut self) -> Result<()>{
		let saga = "Secrets of the Silent Witch";

		let contents: String = get_content("https://en.wikipedia.org/wiki/Secrets_of_the_Silent_Witch")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_executioner(&mut self) -> Result<()>{
		let saga = "The Executioner and Her Way of Life";

		let contents: String = get_content("https://en.wikipedia.org/wiki/The_Executioner_and_Her_Way_of_Life")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_magical_revolution_princess_genius(&mut self) -> Result<()>{
		let saga = "The Magical Revolution of the Reincarnated Princess and the Genius Young Lady";

		let contents: String = get_content("https://en.wikipedia.org/wiki/The_Magical_Revolution_of_the_Reincarnated_Princess_and_the_Genius_Young_Lady")?;
		let all_releases = get_releases_from_html(
			saga,
			contents.as_str(),
			&Selector::parse(".wikitable:nth-of-type(2) tr:has(>th):has(>td)").unwrap(),
		)?;

		add_coming_releases(&all_releases,&mut self.coming);
		self.all.insert(saga.to_string(), all_releases);

		Ok(())
	}

	fn get_bofuri(&mut self) -> Result<()>{
		let saga = "Bofuri";

		let contents: String = get_content("https://en.wikipedia.org/wiki/Bofuri")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let all_releases = Releases::load().unwrap().all;

		let release = all_releases.get("That Time I Got Reincarnated as a Slime - Light Novel").unwrap().get(22).unwrap();
		assert_eq!(release.title, "21");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 07, 08));

		let release = all_releases.get("Villainess_Level_99 - Light Novel").unwrap().get(5).unwrap();
		assert_eq!(release.title, "6");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2024, 07, 31));

		let release = all_releases.get("I'll Become a Villainess Who Goes Down in History - Light Novel").unwrap().get(1).unwrap();
		assert_eq!(release.title, "2");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 06, 10));

		let release = all_releases.get("The Apothecary Diaries - Light Novel").unwrap().get(14).unwrap();
		assert_eq!(release.title, "15");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 09, 12));

		let release = all_releases.get("My Next Life as a Villainess: All Routes Lead to Doom! - Light Novel").unwrap().get(13).unwrap();
		assert_eq!(release.title, "14");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 06, 04));

		let release = all_releases.get("Ascendance of a Bookworm - Hannelore's Fifth Year at the Royal Academy").unwrap().get(0).unwrap();
		assert_eq!(release.title, "34");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 05, 23));

		let release = all_releases.get("Secrets of the Silent Witch").unwrap().get(7).unwrap();
		assert_eq!(release.title, "7");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 11, 11));

		let release = all_releases.get("The Executioner and Her Way of Life").unwrap().get(8).unwrap();
		assert_eq!(release.title, "9");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 08, 12));

		let release = all_releases.get("The Magical Revolution of the Reincarnated Princess and the Genius Young Lady").unwrap().get(7).unwrap();
		assert_eq!(release.title, "8");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2024, 11, 26));

		let release = all_releases.get("Bofuri").unwrap().get(15).unwrap();
		assert_eq!(release.title, "16");
		assert_eq!(release.release_date, NaiveDate::from_ymd_opt(2025, 08, 12));
    }

}