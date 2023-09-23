use tracing::info;

use super::EpisodeResource;
use crate::{
    common::{
        enums::{Downloader, Provider},
        error::{Error, Result},
        state::AppState,
    },
    utils::filename_parser::EpisodeInfo,
};

pub async fn fetch_episodes(state: &AppState, url: &str) -> Result<Vec<EpisodeResource>> {
    let content = state.http_client.get(url).send().await?.bytes().await?;

    match rss::Channel::read_from(&content[..]) {
        Ok(channel) => {
            let res: Vec<_> = channel
                .items()
                .iter()
                .map(|i| EpisodeResource {
                    provider: Provider::Rss,
                    file_url: i.enclosure.clone().unwrap().url,
                    file_size: i.enclosure.clone().unwrap().length.parse().unwrap(),
                    file_downloader: Downloader::Bt,
                    episode: EpisodeInfo::from(i.title.clone().unwrap().as_str()),
                    raw_name: i.title.clone().unwrap(),
                })
                .collect();

            info!("fetched {} episode resources from rss {}", res.len(), url);

            Ok(res)
        }
        Err(e) => Err(Error::Internal(format!(
            "failed to parse rss from url {}, error {}",
            url, e
        ))),
    }
}
