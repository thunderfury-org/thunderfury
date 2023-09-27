use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::common::error::{Error, NotFoundCode, Result};

use super::{
    model::{Request, Response},
    Aria2Trait, Status, TaskOptions,
};

pub struct Client {
    client: reqwest::Client,
    host: String,
    api_token: String,
}

impl Client {
    pub fn new(client: reqwest::Client, host: &str, token: &str) -> Self {
        Self {
            client,
            host: host.to_string(),
            api_token: format!("token:{}", token),
        }
    }

    fn create_request(method: &str, params: Vec<Value>) -> String {
        let request = Request {
            jsonrpc: "2.0",
            id: uuid::Uuid::new_v4().to_string(),
            method: format!("aria2.{}", method),
            params,
        };
        serde_json::to_string(&request).unwrap()
    }

    async fn call_method<T>(&self, method: &str, params: Vec<Value>) -> Result<T>
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
}

#[async_trait::async_trait]
impl Aria2Trait for Client {
    async fn add_uri(&self, uri: &str, options: &TaskOptions) -> Result<String> {
        self.call_method(
            "addUri",
            vec![self.api_token.as_str().into(), vec![uri].into(), options.into()],
        )
        .await
    }

    async fn add_torrent(&self, torrent: &[u8], options: &TaskOptions) -> Result<String> {
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

    async fn tell_status(&self, gid: &str) -> Result<Status> {
        self.call_method("tellStatus", vec![self.api_token.as_str().into(), gid.into()])
            .await
    }

    async fn unpause(&self, gid: &str) -> Result<()> {
        self.call_method("unpause", vec![self.api_token.as_str().into(), gid.into()])
            .await
    }
}
