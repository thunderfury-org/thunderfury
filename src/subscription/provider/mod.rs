use crate::{
    common::{
        enums::{Downloader, Provider},
        error::Result,
        state::AppState,
    },
    utils::filename_parser::EpisodeInfo,
};

pub mod alist;
pub mod rss;

#[derive(Debug)]
pub struct EpisodeResource {
    pub provider: Provider,
    pub file_url: String,
    pub file_size: u64,
    pub file_downloader: Downloader,
    pub episode: EpisodeInfo,
    pub raw_name: String,
}

pub async fn fetch_episode_resources(
    state: &AppState,
    provider: &Provider,
    resource_url: &Option<String>,
) -> Result<Vec<EpisodeResource>> {
    match provider {
        Provider::Alist => alist::fetch_episodes(state, resource_url.as_ref().unwrap()).await,
        Provider::Rss => rss::fetch_episodes(state, resource_url.as_ref().unwrap()).await,
    }
}
