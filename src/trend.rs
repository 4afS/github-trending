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
        user_name: String,
        repo_name: String,
        about: String,
        stars: Stars,
        forks_count: u32,
    ) -> Self {
        Trend {
            user_name: user_name.clone(),
            repo_name: repo_name.clone(),
            repo_url: format!("https://github.com/{}/{}", user_name, repo_name),
            about,
            stars,
            forks_count,
        }
    }
}

#[derive(Serialize)]
pub struct Stars {
    total: u32,
    in_date_range: u32,
}

impl Stars {
    pub fn new(total: u32, in_date_range: u32) -> Self {
        Stars {
            total,
            in_date_range,
        }
    }
}
