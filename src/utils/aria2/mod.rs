use serde::de::DeserializeOwned;
use serde_json::Value;

use self::model::{Request, Response};
use crate::common::{
    error::{Error, NotFoundCode, Result},
    state::AppState,
};

pub use self::model::Status;
pub use self::model::TaskOptions;
pub use self::model::TaskStatus;

mod model;

pub struct Client<'a> {
    client: reqwest::Client,
    host: &'a str,
    api_token: String,
}

impl<'a> Client<'a> {
    fn create_request(method: &str, params: Vec<Value>) -> String {
        let request = Request {
            jsonrpc: "2.0",
            id: uuid::Uuid::new_v4().to_string(),
            method: format!("aria2.{}", method),
            params,
        };
        serde_json::to_string(&request).unwrap()
    }

    pub async fn call_method<T>(&self, method: &str, params: Vec<Value>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let request = Self::create_request(method, params);

        let res: Response = self
            .client
            .post(format!("{}/jsonrpc", self.host))
            .body(request)
            .send()
            .await?
            .json()
            .await?;

        if let Some(v) = res.result {
            serde_json::from_value::<T>(v)
                .map_err(|e| Error::Internal(format!("aria2 jsonrpc parse response error, method {}, {}", method, e)))
        } else if let Some(e) = res.error {
            if e.code == 1 && e.message.contains("not found") {
                Err(Error::NotFound(NotFoundCode::Any, e.message))
            } else {
                Err(Error::Internal(format!(
                    "aria2 jsonrpc error, method {}, error {:?}",
                    method, e
                )))
            }
        } else {
            Err(Error::Internal(format!(
                "aria2 jsonrpc error, method {}, response {:?}",
                method, res
            )))
        }
    }

    pub async fn add_uri(&self, uri: &str, options: &TaskOptions) -> Result<String> {
        self.call_method(
            "addUri",
            vec![self.api_token.as_str().into(), vec![uri].into(), options.into()],
        )
        .await
    }

    pub async fn add_torrent(&self, torrent: &[u8], options: &TaskOptions) -> Result<String> {
        self.call_method(
            "addTorrent",
            vec![
                self.api_token.as_str().into(),
                base64::encode(torrent).into(),
                Vec::<String>::new().into(),
                options.into(),
            ],
        )
        .await
    }

    pub async fn tell_status(&self, gid: &str) -> Result<Status> {
        self.call_method("tellStatus", vec![self.api_token.as_str().into(), gid.into()])
            .await
    }

    pub async fn unpause(&self, gid: &str) -> Result<()> {
        self.call_method("unpause", vec![self.api_token.as_str().into(), gid.into()])
            .await
    }
}

impl<'a> TryFrom<&'a AppState> for Client<'a> {
    type Error = Error;

    fn try_from(state: &'a AppState) -> Result<Self> {
        match &state.config.get_downloader_config("aria2") {
            Some(c) => {
                if !c.contains_key("host") {
                    return Err(Error::Internal("downloader aria2 host not set".to_string()));
                }
                if !c.contains_key("token") {
                    return Err(Error::Internal("downloader aria2 token not set".to_string()));
                }

                Ok(Client {
                    client: state.http_client.clone(),
                    host: c.get("host").unwrap(),
                    api_token: format!("token:{}", c.get("token").unwrap()),
                })
            }
            None => Err(Error::Internal("downloader aria2 config not set".to_string())),
        }
    }
}
