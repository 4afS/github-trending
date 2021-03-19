use nipper::{Document, Selection};

use crate::{
    error::TrendError,
    trend::{Stars, Trend},
};

pub fn get_trend(document: Document) -> anyhow::Result<Vec<Trend>> {
    let mut result = Vec::new();

    for article in document.select("article.Box-row").iter() {
        let user_repo = username_and_repo(&article)
            .ok_or_else(|| TrendError::new("username and repository not found"))?;
        result.push(Trend::new(
            user_repo.0,
            user_repo.1,
            about(&article).ok_or_else(|| TrendError::new("about repository nod found"))?,
            Stars::new(
                stars_count(&article)
                    .ok_or_else(|| TrendError::new("total stars count not found"))?,
                stars_count_in(&article)
                    .ok_or_else(|| TrendError::new("stars count in date range not found"))?,
            ),
            forks_count(&article).ok_or_else(|| TrendError::new("forks count not found"))?,
        ));
    }

    Ok(result)
}

fn username_and_repo(article: &Selection) -> Option<(String, String)> {
    let elements = article
        .select("h1:nth-child(2) > a:nth-child(1)")
        .text()
        .split('/')
        .map(|e| e.trim().to_string())
        .collect::<Vec<_>>();
    return Some((elements.get(0)?.to_string(), elements.get(1)?.to_string()));
}

fn about(article: &Selection) -> Option<String> {
    let about = article.select("p:nth-child(3)").text().trim().to_string();
    if about.is_empty() {
        None
    } else {
        Some(about)
    }
}

fn number(article: &Selection, selector: &str) -> Option<u32> {
    let total_star_count = article
        .select(selector)
        .text()
        .to_string()
        .chars()
        .filter(|e| e.is_digit(10))
        .collect::<String>();
    if total_star_count.is_empty() {
        None
    } else {
        Some(total_star_count.parse::<u32>().ok()?)
    }
}

fn stars_count(article: &Selection) -> Option<u32> {
    number(article, "div:nth-child(4) > a:nth-child(2)")
}

fn forks_count(article: &Selection) -> Option<u32> {
    number(article, "div:nth-child(4) > a:nth-child(3)")
}

fn stars_count_in(article: &Selection) -> Option<u32> {
    number(article, "div:nth-child(4) > span:nth-child(5)")
}
