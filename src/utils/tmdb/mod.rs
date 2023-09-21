use std::fmt::Display;

use reqwest::{IntoUrl, StatusCode};
use serde::de::DeserializeOwned;

use crate::common::{
    error::{Error, NotFoundCode, Result},
    state::AppState,
};
use model::{SearchTvResponse, SearchTvResult, SeasonDetail, TvDetail};

pub mod model;

const TMDB_HOST: &str = "https://api.themoviedb.org/3";

pub struct Client<'a> {
    client: reqwest::Client,
    api_key: &'a str,
}

impl<'a> Client<'a> {
    async fn get<U: IntoUrl + Display, T: DeserializeOwned>(
        &self,
        url: U,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<T> {
        let mut request_query = vec![("language", "zh-CN"), ("api_key", self.api_key)];
        if let Some(q) = query {
            request_query.extend(q);
        }

        let response = self
            .client
            .get(format!("{}{}", TMDB_HOST, url))
            .query(&request_query)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            return serde_json::from_str(&body)
                .map_err(|e| Error::Internal(format!("http get {url} failed, decode body failed, {e}, body: {body}")));
        }

        match status {
            StatusCode::NOT_FOUND => Err(Error::NotFound(NotFoundCode::Any, "tmdb not found".to_string())),
            _ => Err(Error::Internal(format!(
                "http get {url} failed, status: {status}, body: {body}"
            ))),
        }
    }

    pub async fn get_tv_detail(&self, tv_id: u32) -> Result<TvDetail> {
        match self.get(format!("/tv/{}", tv_id), None).await {
            Ok(detail) => Ok(detail),
            Err(Error::NotFound(_, _)) => Err(Error::NotFound(
                NotFoundCode::Tv,
                format!("can not find tv {} in tmdb", tv_id),
            )),
            Err(e) => Err(e),
        }
    }

    pub async fn get_season_detail(&self, tv_id: u32, season_number: u32) -> Result<SeasonDetail> {
        match self.get(format!("/tv/{}/season/{}", tv_id, season_number), None).await {
            Ok(detail) => Ok(detail),
            Err(Error::NotFound(_, _)) => Err(Error::NotFound(
                NotFoundCode::Season,
                format!("can not find season {} of tv {} in tmdb", season_number, tv_id),
            )),
            Err(e) => Err(e),
        }
    }

    pub async fn search_tv(&self, query: &str) -> Result<Vec<SearchTvResult>> {
        let response: SearchTvResponse = self
            .get(
                "/search/tv",
                Some(vec![("query", query), ("include_adult", "true"), ("page", "1")]),
            )
            .await?;
        Ok(response.results)
    }
}

impl<'a> TryFrom<&'a AppState> for Client<'a> {
    type Error = Error;

    fn try_from(state: &'a AppState) -> Result<Self> {
        match &state.config.get_library_config().tmdb_api_key {
            Some(api_key) => Ok(Self {
                client: state.http_client.clone(),
                api_key,
            }),
            None => Err(Error::Internal("tmdb api key not set".to_string())),
        }
    }
}
