use nipper::{Document, Selection};

use crate::{
    error::TrendError,
    trend::{Stars, Trend},
};

pub fn get_trend(document: Document) -> anyhow::Result<Vec<Trend>> {
    let mut result = Vec::new();

    for article in document.select("article.Box-row").iter() {
        let (username, repository) = username_and_repository(&article)?;
        result.push(Trend::new(
            &username,
            &repository,
            &about(&article)?,
            Stars::new(stars_count(&article)?, stars_count_in(&article)?),
            forks_count(&article)?,
        ));
    }

    Ok(result)
}

fn username_and_repository(article: &Selection) -> anyhow::Result<(String, String)> {
    let elements = article
        .select("h1:nth-child(2) > a:nth-child(1)")
        .text()
        .split('/')
        .map(|e| e.trim().to_string())
        .collect::<Vec<_>>();

    match (elements.get(0), elements.get(1)) {
        (None, None) => Err(TrendError::NotFound("username and repository".to_string()).into()),
        (_, None) => Err(TrendError::NotFound("repository".to_string()).into()),
        (None, _) => Err(TrendError::NotFound("username".to_string()).into()),
        (Some(username), Some(repository)) => Ok((username.to_string(), repository.to_string())),
    }
}

fn about(article: &Selection) -> anyhow::Result<String> {
    let about = article.select("p:nth-child(3)").text().trim().to_string();
    if about.is_empty() {
        Err(TrendError::NotFound("about".to_string()).into())
    } else {
        Ok(about)
    }
}

fn number(article: &Selection, selector: &str, param_type: &str) -> anyhow::Result<u32> {
    let total_star_count = article
        .select(selector)
        .text()
        .to_string()
        .chars()
        .filter(|e| e.is_digit(10))
        .collect::<String>();

    if total_star_count.is_empty() {
        return Err(TrendError::NotFound(param_type.to_string()).into());
    }

    if let Ok(number) = total_star_count.parse::<u32>() {
        Ok(number)
    } else {
        Err(TrendError::ParseError.into())
    }
}

fn stars_count(article: &Selection) -> anyhow::Result<u32> {
    number(article, "div:nth-child(4) > a:nth-child(2)", "stars_count")
}

fn forks_count(article: &Selection) -> anyhow::Result<u32> {
    number(article, "div:nth-child(4) > a:nth-child(3)", "forks_count")
}

fn stars_count_in(article: &Selection) -> anyhow::Result<u32> {
    number(
        article,
        "div:nth-child(4) > span:nth-child(5)",
        "stars_count_in",
    )
}
