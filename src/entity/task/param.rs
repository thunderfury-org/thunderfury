use serde::{Deserialize, Serialize};

use crate::common::enums::{Downloader, FileType, MediaType};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadMediaFileParam {
    pub media_type: MediaType,
    pub media_id: i32,
    pub year: i32,
    pub original_name: String,
    pub season_number: Option<i32>,
    pub episode_number: Option<i32>,
    pub file_type: FileType,
    pub file_ext: String,
    pub file_url: String,
    pub file_downloader: Downloader,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type", content = "param")]
pub enum PushMessageParam {
    EpisodeDownloaded {
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    },
    MovieDownloaded {
        movie_id: i32,
    },
}
