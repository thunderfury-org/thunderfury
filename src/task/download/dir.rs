use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    common::enums::{FileType, MediaType},
    common::error::{Error, Result},
    entity::task::param::DownloadMediaFileParam,
    utils::{filename_parser::EpisodeInfo, fs},
};

const STORE_DIR: &str = "/store";
const LIBRARY_DIR: &str = "/library";

impl DownloadMediaFileParam {
    /// Get file path in library dir
    pub fn get_library_file_path(&self) -> String {
        format!("{}{}/{}", LIBRARY_DIR, self.get_file_dir(), self.get_file_name())
    }

    pub fn get_library_file_dir(&self) -> String {
        format!("{}{}", LIBRARY_DIR, self.get_file_dir())
    }

    /// Format
    ///
    /// - tv
    ///   - /tv/{original_name} ({year})/Season {:02}
    /// - movie
    ///   - /movie/{original_name} ({year})
    fn get_file_dir(&self) -> String {
        match self.media_type {
            MediaType::Tv => format!(
                "/tv/{} ({})/Season {:02}",
                self.original_name,
                self.year,
                self.season_number.unwrap(),
            ),
            MediaType::Movie => format!("/movie/{} ({})", self.original_name, self.year),
        }
    }

    /// Format: {basename}.{ext}
    pub fn get_file_name(&self) -> String {
        let base = self.get_base_file_name();

        if !self.file_ext.is_empty() {
            format!("{}.{}", base, self.file_ext)
        } else {
            base
        }
    }

    /// Format
    ///
    /// - tv: S{:02}E{:02}
    /// - movie: {original_name} ({year})
    pub fn get_base_file_name(&self) -> String {
        match self.media_type {
            MediaType::Tv => format!(
                "S{:02}E{:02}",
                self.season_number.unwrap(),
                self.episode_number.unwrap(),
            ),
            MediaType::Movie => format!("{} ({})", self.original_name, self.year),
        }
    }
}

pub fn get_download_dir(library_root: &str, task_param: &DownloadMediaFileParam) -> String {
    match task_param.media_type {
        MediaType::Tv => format!(
            "{}{}{}/{}",
            library_root,
            STORE_DIR,
            task_param.get_file_dir(),
            task_param.get_base_file_name()
        ),
        MediaType::Movie => format!("{}{}{}", library_root, STORE_DIR, task_param.get_file_dir()),
    }
}

pub fn link_downloaded_files(
    library_root: &str,
    task_param: &DownloadMediaFileParam,
) -> Result<DownloadMediaFileParam> {
    let download_dir = get_download_dir(library_root, task_param);

    let mut files: HashMap<FileType, Vec<(PathBuf, EpisodeInfo)>> = HashMap::new();

    for entry in fs::list_all_files(&download_dir)? {
        let info = EpisodeInfo::from(entry.file_name().unwrap().to_str().unwrap());
        if info.file_type == FileType::Unknown {
            // skip unknown file
            continue;
        }
        files.entry(info.file_type).or_default().push((entry, info));
    }

    if let Some(subtitle_files) = files.get(&FileType::Subtitle) {
        link_subtitle_files(library_root, task_param, subtitle_files)?
    }

    let mut res = (*task_param).clone();

    match files.get(&FileType::Video) {
        Some(video_files) => {
            if video_files.len() > 1 {
                return Err(Error::Internal(format!(
                    "more than one video file found in dir {}",
                    download_dir
                )));
            }

            let f = &video_files[0];

            res.file_type = FileType::Video;
            res.file_ext = f.1.extension.as_ref().unwrap().to_owned();

            fs::hard_link_file(
                &f.0,
                format!("{}{}", library_root, res.get_library_file_path()).as_str(),
            )?;
        }
        None => return Err(Error::Internal(format!("no video files found in dir {}", download_dir))),
    }

    Ok(res)
}

fn link_subtitle_files(
    library_root: &str,
    task_param: &DownloadMediaFileParam,
    subtitle_files: &[(PathBuf, EpisodeInfo)],
) -> Result<()> {
    if subtitle_files.is_empty() {
        return Ok(());
    }

    let mut group_by_lang: HashMap<String, Vec<&(PathBuf, EpisodeInfo)>> = HashMap::new();
    for entry in subtitle_files {
        match entry.1.subtitles.as_ref() {
            Some(subtitles) => {
                let key = subtitles.join("-");
                group_by_lang.entry(key).or_default().push(entry);
            }
            None => group_by_lang.entry("".to_owned()).or_default().push(entry),
        }
    }

    for (lang, subtitle_files) in group_by_lang {
        for (index, entry) in subtitle_files.iter().enumerate() {
            fs::hard_link_file(
                &entry.0,
                format!(
                    "{}{}/{}.{}{}",
                    library_root,
                    task_param.get_library_file_dir(),
                    task_param.get_base_file_name(),
                    get_subtitle_filename_lang(lang.as_str(), index),
                    entry.1.extension.as_ref().unwrap(),
                )
                .as_str(),
            )?;
        }
    }

    Ok(())
}

fn get_subtitle_filename_lang(lang: &str, index: usize) -> String {
    if index == 0 {
        return format!("{}.", lang);
    }

    if lang.is_empty() {
        return format!("{}.", index);
    }

    format!("{}.{}.", lang, index)
}

pub fn is_file_downloaded(library_root: &str, task_param: &DownloadMediaFileParam) -> Result<bool> {
    let base_path = format!("{library_root}{}", task_param.get_library_file_dir());
    if !Path::new(&base_path).exists() {
        return Ok(false);
    }

    let name_prefix = format!("{}.", task_param.get_base_file_name());

    let files = std::fs::read_dir(base_path)?;
    let count = files
        .filter_map(std::result::Result::ok)
        .filter(|d| d.file_name().to_str().unwrap().starts_with(&name_prefix))
        .count();

    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::get_download_dir;
    use crate::{
        common::enums::{Downloader, FileType, MediaType},
        entity::task::param::DownloadMediaFileParam,
    };

    const LIBRARY_ROOT: &str = "/media";

    #[test]
    fn test_tv_dir() {
        let task_param = &DownloadMediaFileParam {
            media_type: MediaType::Tv,
            media_id: 1,
            original_name: "abc".to_owned(),
            year: 2023,
            season_number: Some(1),
            episode_number: Some(1),
            file_type: FileType::Video,
            file_ext: "mp4".to_owned(),
            file_url: "".to_owned(),
            file_downloader: Downloader::Alist,
        };
        assert_eq!(
            get_download_dir(LIBRARY_ROOT, task_param),
            "/media/store/tv/abc (2023)/Season 01/S01E01"
        );
        assert_eq!(
            task_param.get_library_file_path(),
            "/library/tv/abc (2023)/Season 01/S01E01.mp4"
        );
    }

    #[test]
    fn test_movie_dir() {
        let task_param = &DownloadMediaFileParam {
            media_type: MediaType::Movie,
            media_id: 1,
            original_name: "abc".to_owned(),
            year: 2023,
            season_number: None,
            episode_number: None,
            file_type: FileType::Video,
            file_ext: "mp4".to_owned(),
            file_url: "".to_owned(),
            file_downloader: Downloader::Bt,
        };
        assert_eq!(
            get_download_dir(LIBRARY_ROOT, task_param),
            "/media/store/movie/abc (2023)"
        );
        assert_eq!(
            task_param.get_library_file_path(),
            "/library/movie/abc (2023)/abc (2023).mp4"
        );
    }

    #[test]
    fn test_subtitle_filename_lang() {
        assert_eq!(".", super::get_subtitle_filename_lang("", 0));
        assert_eq!("1.", super::get_subtitle_filename_lang("", 1));
        assert_eq!("zh-CN.", super::get_subtitle_filename_lang("zh-CN", 0));
        assert_eq!("zh-CN.1.", super::get_subtitle_filename_lang("zh-CN", 1));
    }
}
