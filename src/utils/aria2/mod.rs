use crate::common::{
    error::{Error, Result},
    state::AppState,
};

pub use self::model::Status;
pub use self::model::TaskOptions;
pub use self::model::TaskStatus;

mod inner;
mod mock;
mod model;

#[async_trait::async_trait]
pub trait Aria2Trait {
    async fn add_uri(&self, uri: &str, options: &TaskOptions) -> Result<String>;
    async fn add_torrent(&self, torrent: &[u8], options: &TaskOptions) -> Result<String>;
    async fn tell_status(&self, gid: &str) -> Result<Status>;
    async fn unpause(&self, gid: &str) -> Result<()>;
}

pub enum Client {
    InnerClient(inner::Client),
    MockClient(mock::Client),
}

#[async_trait::async_trait]
impl Aria2Trait for Client {
    async fn add_uri(&self, uri: &str, options: &TaskOptions) -> Result<String> {
        match self {
            Self::InnerClient(c) => c.add_uri(uri, options).await,
            Self::MockClient(c) => c.add_uri(uri, options).await,
        }
    }

    async fn add_torrent(&self, torrent: &[u8], options: &TaskOptions) -> Result<String> {
        match self {
            Self::InnerClient(c) => c.add_torrent(torrent, options).await,
            Self::MockClient(c) => c.add_torrent(torrent, options).await,
        }
    }

    async fn tell_status(&self, gid: &str) -> Result<Status> {
        match self {
            Self::InnerClient(c) => c.tell_status(gid).await,
            Self::MockClient(c) => c.tell_status(gid).await,
        }
    }

    async fn unpause(&self, gid: &str) -> Result<()> {
        match self {
            Self::InnerClient(c) => c.unpause(gid).await,
            Self::MockClient(c) => c.unpause(gid).await,
        }
    }
}

impl TryFrom<&AppState> for Client {
    type Error = Error;

    fn try_from(state: &AppState) -> Result<Self> {
        match &state.config.get_downloader_config("aria2") {
            Some(c) => {
                let mock_data_dir = c.get("mock_data_dir").map_or_else(|| "", |s| s.trim());
                if mock_data_dir.is_empty() {
                    if !c.contains_key("host") {
                        return Err(Error::Internal("downloader aria2 host not set".to_string()));
                    }
                    if !c.contains_key("token") {
                        return Err(Error::Internal("downloader aria2 token not set".to_string()));
                    }

                    Ok(Client::InnerClient(inner::Client::new(
                        state.http_client.clone(),
                        c.get("host").unwrap(),
                        c.get("token").unwrap(),
                    )))
                } else {
                    Ok(Client::MockClient(mock::Client::new(mock_data_dir)))
                }
            }
            None => Err(Error::Internal("downloader aria2 config not set".to_string())),
        }
    }
}
