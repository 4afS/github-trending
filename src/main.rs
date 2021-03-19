use github_trending_api::get_trend::get_trend;
use github_trending_api::url::{construct_url, DateRange};

use nipper::Document;

fn main() -> anyhow::Result<()> {
    let language = "rust";
    let date_range = DateRange::Today;

    let url = construct_url(None, language, &date_range);

    let response = reqwest::blocking::get(url)?.text()?;

    let trends = get_trend(Document::from(&response))?;

    print!("{}", serde_json::to_string(&trends)?);

    Ok(())
}
