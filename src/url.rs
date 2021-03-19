use derive_more::Display;

#[derive(Display)]
pub enum DateRange {
    #[display(fmt = "daily")]
    Today,
    #[display(fmt = "weekly")]
    ThisWeek,
    #[display(fmt = "monthly")]
    ThisMonth,
}

pub fn construct_url(
    spoken_language: Option<&str>,
    language: &str,
    date_range: &DateRange,
) -> String {
    return format!(
        "https://github.com/trending/{}?since={}&spoken_language_code={}",
        language,
        date_range.to_string(),
        spoken_language.unwrap_or(""),
    );
}

#[test]
fn test_construct_url() {
    let spoken_language = "en";
    let language = "rust";

    let daily = construct_url(Some(spoken_language), language, &DateRange::Today);
    let weekly = construct_url(Some(spoken_language), language, &DateRange::ThisWeek);
    let monthly = construct_url(Some(spoken_language), language, &DateRange::ThisMonth);

    let no_lang = construct_url(None, language, &DateRange::Today);

    assert_eq!(
        daily,
        "https://github.com/trending/rust?since=daily&spoken_language_code=en",
    );

    assert_eq!(
        weekly,
        "https://github.com/trending/rust?since=weekly&spoken_language_code=en",
    );

    assert_eq!(
        monthly,
        "https://github.com/trending/rust?since=monthly&spoken_language_code=en",
    );

    assert_eq!(
        no_lang,
        "https://github.com/trending/rust?since=daily&spoken_language_code=",
    );
}
