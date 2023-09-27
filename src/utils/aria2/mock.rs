use std::path::Path;

use super::{Aria2Trait, Status, TaskOptions};
use crate::{
    common::error::{Error, NotFoundCode, Result},
    utils::fs,
};

pub struct Client {
    data_dir: String,
}

impl Client {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Aria2Trait for Client {
    async fn add_uri(&self, _uri: &str, options: &TaskOptions) -> Result<String> {
        let filepath = format!(
            "{}/{}",
            options.dir.as_deref().unwrap(),
            options.out.as_deref().unwrap()
        );
        let gid = sha256::digest(&filepath);

        fs::create_file_if_not_exists(format!("{}/{}", self.data_dir, gid).as_str())?;
        fs::create_file_if_not_exists(&filepath)?;

        Ok(gid)
    }

    async fn add_torrent(&self, torrent: &[u8], options: &TaskOptions) -> Result<String> {
        let gid = sha256::digest(torrent);

        fs::create_file_if_not_exists(format!("{}/{}", self.data_dir, gid).as_str())?;
        fs::create_file_if_not_exists(format!("{}/{}.mp4", options.dir.as_deref().unwrap(), gid).as_str())?;

        Ok(gid)
    }

    async fn tell_status(&self, gid: &str) -> Result<Status> {
        let path = format!("{}/{}", self.data_dir, gid);
        let p = Path::new(&path);
        if !p.exists() {
            return Err(Error::NotFound(NotFoundCode::Any, "not found".to_string()));
        }

        Ok(Status {
            gid: gid.to_string(),
            status: super::TaskStatus::Complete,
            total_length: "1".to_string(),
            completed_length: "1".to_string(),
            download_speed: "1".to_string(),
            error_code: None,
            error_message: None,
        })
    }

    async fn unpause(&self, _gid: &str) -> Result<()> {
        Ok(())
    }
}
