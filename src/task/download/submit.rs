use super::dir;
use crate::{
    common::{enums, error::Result, state::AppState},
    entity::task::param::DownloadMediaFileParam,
    utils::{
        alist,
        aria2::{self, Aria2Trait},
        fs, http,
    },
};

pub async fn submit_task(state: &AppState, task_param: &DownloadMediaFileParam) -> Result<String> {
    match task_param.file_downloader {
        enums::Downloader::Bt => submit_bt_download_task(state, task_param).await,
        enums::Downloader::Alist => submit_http_download_task(state, task_param).await,
    }
}

async fn submit_http_download_task(state: &AppState, task_param: &DownloadMediaFileParam) -> Result<String> {
    let f = alist::Client::try_from(state)?.get(&task_param.file_url).await?;

    aria2::Client::try_from(state)?
        .add_uri(
            &f.raw_url,
            &aria2::TaskOptions {
                dir: Some(dir::get_download_dir(state.config.get_library_root(), task_param)),
                out: Some(task_param.get_file_name()),
                r#continue: Some(true),
                allow_overwrite: Some(false),
                auto_file_renaming: Some(false),
                ..Default::default()
            },
        )
        .await
}

async fn submit_bt_download_task(state: &AppState, task_param: &DownloadMediaFileParam) -> Result<String> {
    let torrent = http::download_file(state.http_client.clone(), &task_param.file_url).await?;

    let download_dir = dir::get_download_dir(state.config.get_library_root(), task_param);
    fs::create_dir_all(&download_dir)?;

    aria2::Client::try_from(state)?
        .add_torrent(
            torrent.as_ref(),
            &aria2::TaskOptions {
                dir: Some(download_dir),
                rpc_save_upload_metadata: Some(true),
                ..Default::default()
            },
        )
        .await
}
