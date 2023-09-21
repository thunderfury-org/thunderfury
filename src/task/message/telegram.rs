use serde_json::json;

use crate::common::{
    error::{Error, NotFoundCode, Result},
    state::AppState,
};

pub async fn send_message(state: &AppState, message: &str) -> Result<()> {
    let conf = state
        .config
        .get_message_channel_config("telegram")
        .ok_or(Error::NotFound(
            NotFoundCode::Config,
            "telegram message channel config not found".to_owned(),
        ))?;

    let token = conf.get("token").ok_or(Error::NotFound(
        NotFoundCode::Config,
        "telegram token not found".to_owned(),
    ))?;
    let chat_id = conf.get("chat_id").ok_or(Error::NotFound(
        NotFoundCode::Config,
        "telegram chat_id not found".to_owned(),
    ))?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let resp = state
        .http_client
        .post(url)
        .json(&json!({"chat_id": chat_id, "text": message}))
        .send()
        .await?;

    if !resp.status().is_success() {
        Err(Error::Internal(format!(
            "send message to telegram failed, {}",
            resp.text().await?,
        )))
    } else {
        Ok(())
    }
}
