use github_trending::url::{construct_url, DateRange};
use github_trending::{error::ErrorResponse, get_trend::get_trend};

use nipper::Document;

fn main() -> anyhow::Result<()> {
    let language = "rust";
    let date_range = DateRange::Today;

    let url = construct_url(None, language, &date_range);

    let response = reqwest::blocking::get(url)?.text()?;

    match get_trend(language, Document::from(&response)) {
        Ok(trend_repos) => print!("{}", serde_json::to_string(&trend_repos)?),
        Err(e) => print!(
            "{}",
            serde_json::to_string(&ErrorResponse::new(e.to_string()))?
        ),
    }

    Ok(())
}
