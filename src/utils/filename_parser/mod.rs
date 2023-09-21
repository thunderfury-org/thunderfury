use std::collections::HashSet;

use serde::Deserialize;

use crate::entity::enums::FileType;
use title::Title;

mod episode;
pub mod lang;
pub mod title;

impl From<&str> for FileType {
    fn from(val: &str) -> Self {
        lazy_static::lazy_static! {
            static ref VIDEO_EXT: HashSet<&'static str> = HashSet::from([
                "3g2", "3gp", "3gp2", "asf", "avi", "divx", "flv", "iso",
                "m4v", "mk2", "mk3d", "mka", "mkv", "mov", "mp4", "mp4a",
                "mpeg", "mpg", "ogg", "ogm", "ogv", "qt", "ra", "ram",
                "rm", "ts", "m2ts", "vob", "wav", "webm", "wma", "wmv"
            ]);

            static ref SUBTITLE_EXT: HashSet<&'static str> = HashSet::from([
                "srt", "idx", "sub", "ssa", "ass"
            ]);
        }

        if VIDEO_EXT.contains(val) {
            return FileType::Video;
        }
        if SUBTITLE_EXT.contains(val) {
            return FileType::Subtitle;
        }

        FileType::Unknown
    }
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EpisodeInfo {
    pub file_type: FileType,
    pub extension: Option<String>,
    pub titles: Option<Vec<Title>>,
    pub release_group: Option<String>,
    pub season_number: Option<u32>,
    pub episode_number: Option<u32>,
    pub resolution: Option<String>,
    pub subtitles: Option<Vec<String>>,
}
