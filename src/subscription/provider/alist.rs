use tracing::info;

use crate::{
    common::{error::Result, state::AppState},
    entity::enums,
    utils::{alist, filename_parser::EpisodeInfo},
};

use super::EpisodeResource;

pub async fn fetch_episodes(state: &AppState, url: &str) -> Result<Vec<EpisodeResource>> {
    info!("start to fetch episode resources from alist {}", url);

    let files = alist::Client::try_from(state)?.list(url).await?;

    let res: Vec<_> = files
        .into_iter()
        .filter(|f| !f.is_dir)
        .map(|f| EpisodeResource {
            provider: enums::Provider::Alist,
            file_url: f.path,
            file_size: f.size,
            file_downloader: enums::Downloader::Alist,
            episode: EpisodeInfo::from(f.name.as_str()),
            raw_name: f.name,
        })
        .filter(|r| r.episode.file_type != enums::FileType::Unknown)
        .collect();

    info!("fetched {} episode resources from alist {}", res.len(), url);

    Ok(res)
}
