use serde::{Deserialize, Serialize};

use crate::{
    common::error::Result,
    entity::{enums, task},
};

impl task::Model {
    pub fn deserialize_param<'a, T>(&'a self) -> Result<T>
    where
        T: serde::de::Deserialize<'a>,
    {
        Ok(serde_json::from_str(self.param.as_str())?)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadMediaFileParam {
    pub media_type: enums::MediaType,
    pub media_id: i32,
    pub year: i32,
    pub original_name: String,
    pub season_number: Option<i32>,
    pub episode_number: Option<i32>,
    pub file_type: enums::FileType,
    pub file_ext: String,
    pub file_url: String,
    pub file_downloader: enums::Downloader,
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
