use nipper::{Document, Selection};

use crate::model::{error::TrendError, repository::Repository};

pub fn get_repositories(language: &str, document: Document) -> anyhow::Result<Vec<Repository>> {
    if !is_given_language_exist(language, &document) {
        return Err(TrendError::GivenLanguageIsNotExist.into());
    }

    let mut result = Vec::new();
    for article in document.select("article.Box-row").iter() {
        let (username, repository) = username_and_repository(&article)?;
        result.push(Repository::new(&username, &repository));
    }

    Ok(result)
}

fn is_given_language_exist(lang: &str, article: &Document) -> bool {
    let obtained_lang = article
        .select("#select-menu-language > summary:nth-child(1) > span:nth-child(1)")
        .text()
        .to_string();

    lang.to_lowercase().eq(&obtained_lang.trim().to_lowercase())
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
