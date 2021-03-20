use github_trending::trend::get_repositories;
use github_trending::url::{construct_url, DateRange};

use nipper::Document;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let language = "haskell";
    let date_range = DateRange::Today;

    let url = construct_url(None, language, &date_range);

    let response = reqwest::get(url).await?.text().await?;

    let repos = get_repositories(language, Document::from(&response))?;
    for repo in repos {
        let repo_model: octocrab::models::Repository = octocrab::instance()
            .get(
                format!("https://api.github.com/repos/{}", repo.path()),
                None::<&()>,
            )
            .await?;
        println!("{:?}", repo_model.stargazers_count);
    }

    Ok(())
}
