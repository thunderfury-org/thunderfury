use bytes::Bytes;

use crate::common::error::{Error, Result};

pub async fn download_file(client: reqwest::Client, url: &str) -> Result<Bytes> {
    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        return Err(Error::Internal(format!(
            "download file failed, status: {}, url {}",
            resp.status(),
            url
        )));
    }

    Ok(resp.bytes().await?)
}
