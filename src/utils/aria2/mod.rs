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

pub enum Client {
    InnerClient(inner::Client),
    MockClient(mock::Client),
}

impl TryFrom<&AppState> for Client {
    type Error = Error;

    fn try_from(state: &AppState) -> Result<Self> {
        match &state.config.get_downloader_config("aria2") {
            Some(c) => {
                c.get("mock_data_dir").as_deref()
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
            }
            None => Err(Error::Internal("downloader aria2 config not set".to_string())),
        }
    }
}
