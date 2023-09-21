use crate::{
    common::{error::Result, state::AppState},
    entity::enums,
    utils::filename_parser::EpisodeInfo,
};

pub mod alist;
pub mod rss;

#[derive(Debug)]
pub struct EpisodeResource {
    pub provider: enums::Provider,
    pub file_url: String,
    pub file_size: u64,
    pub file_downloader: enums::Downloader,
    pub episode: EpisodeInfo,
    pub raw_name: String,
}

pub async fn fetch_episode_resources(
    state: &AppState,
    provider: &enums::Provider,
    resource_url: &Option<String>,
) -> Result<Vec<EpisodeResource>> {
    match provider {
        enums::Provider::Alist => alist::fetch_episodes(state, resource_url.as_ref().unwrap()).await,
        enums::Provider::Rss => rss::fetch_episodes(state, resource_url.as_ref().unwrap()).await,
    }
}
