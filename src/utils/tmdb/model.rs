use serde::{Deserialize, Deserializer};

fn nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Debug, Deserialize)]
pub struct TvDetail {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub adult: bool,
    pub first_air_date: String,
    pub in_production: bool,
    pub last_air_date: String,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    #[serde(deserialize_with = "nullable_string")]
    pub poster_path: String,
    #[serde(deserialize_with = "nullable_string")]
    pub backdrop_path: String,
}

#[derive(Debug, Deserialize)]
pub struct SeasonDetail {
    pub id: u32,
    pub season_number: u32,
    pub air_date: String,
    pub overview: String,
    #[serde(deserialize_with = "nullable_string")]
    pub poster_path: String,
    pub episodes: Vec<EpisodeDetail>,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeDetail {
    pub id: u32,
    pub season_number: u32,
    pub episode_number: u32,
    pub name: String,
    pub overview: String,
    #[serde(deserialize_with = "nullable_string")]
    pub air_date: String,
    #[serde(deserialize_with = "nullable_string")]
    pub still_path: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchTvResponse {
    pub results: Vec<SearchTvResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchTvResult {
    pub id: u32,
    pub name: String,
    pub first_air_date: String,
    pub overview: String,
    #[serde(deserialize_with = "nullable_string")]
    pub poster_path: String,
    #[serde(deserialize_with = "nullable_string")]
    pub backdrop_path: String,
}
