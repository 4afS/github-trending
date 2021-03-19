use derive_new::new;
use serde::Serialize;

#[derive(Serialize)]
pub struct Trend {
    user_name: String,
    repo_name: String,
    repo_url: String,
    about: String,
    stars: Stars,
    forks_count: u32,
}

impl Trend {
    pub fn new(
        user_name: &str,
        repo_name: &str,
        about: &str,
        stars: Stars,
        forks_count: u32,
    ) -> Self {
        Trend {
            user_name: user_name.to_string(),
            repo_name: repo_name.to_string(),
            repo_url: format!("https://github.com/{}/{}", user_name, repo_name),
            about: about.to_string(),
            stars,
            forks_count,
        }
    }
}

#[test]
fn test_trend_new() {
    assert_eq!(
        Trend::new("username", "repository", "", Stars::new(0, 0), 0,).repo_url,
        "https://github.com/username/repository"
    )
}

#[derive(Serialize, new)]
pub struct Stars {
    total: u32,
    in_date_range: u32,
}
