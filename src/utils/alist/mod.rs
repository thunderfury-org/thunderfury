use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::common::{
    error::{Error, NotFoundCode, Result},
    state::AppState,
};
use model::{File, GetRequest, ListRequest, ListResponse, ResponseModel};

mod model;

pub struct Client<'a> {
    client: reqwest::Client,
    host: &'a str,
    api_token: &'a str,
}

impl<'a> Client<'a> {
    async fn post<I: Serialize, T: DeserializeOwned>(&self, url: &str, json: &I) -> Result<T> {
        let response = self
            .client
            .post(format!("{}{}", self.host, url))
            .header("Authorization", self.api_token)
            .json(json)
            .send()
            .await?;

        if !response.status().is_success() {
            let u = response.url().to_string();
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|e| format!("parse response body to string error, {}", e));

            return Err(Error::Internal(format!(
                "http post {u} failed, status: {status}, body: {body}"
            )));
        }

        let r = response.json::<ResponseModel<T>>().await?;
        if r.code != 200 {
            if r.message.contains("not found") {
                return Err(Error::NotFound(NotFoundCode::Any, r.message));
            } else {
                return Err(Error::Internal(format!(
                    "http post failed, code: {}, message: {}",
                    r.code, r.message
                )));
            }
        }

        Ok(r.data.unwrap())
    }

    pub async fn list(&self, path: &str) -> Result<Vec<File>> {
        let mut response: ListResponse = self
            .post(
                "/api/fs/list",
                &ListRequest {
                    path,
                    page: 1,
                    per_page: 0,
                    refresh: true,
                    password: "",
                },
            )
            .await?;

        for f in response.content.as_mut_slice() {
            f.path = Path::new(path).join(f.name.as_str()).to_str().unwrap().to_string();
        }

        Ok(response.content)
    }

    pub async fn get(&self, url: &str) -> Result<File> {
        let file: File = self
            .post(
                "/api/fs/get",
                &GetRequest {
                    path: url,
                    password: "",
                },
            )
            .await?;

        Ok(file)
    }
}

impl<'a> TryFrom<&'a AppState> for Client<'a> {
    type Error = Error;

    fn try_from(state: &'a AppState) -> Result<Self> {
        match state.config.get_provider_config("alist") {
            Some(c) => {
                if !c.contains_key("host") {
                    return Err(Error::Internal("provider alist host not set".to_string()));
                }
                if !c.contains_key("token") {
                    return Err(Error::Internal("provider alist token not set".to_string()));
                }

                Ok(Client {
                    client: state.http_client.clone(),
                    host: c.get("host").unwrap(),
                    api_token: c.get("token").unwrap(),
                })
            }
            None => Err(Error::Internal("provider alist config not set".to_string())),
        }
    }
}
