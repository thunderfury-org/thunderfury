use sea_orm::TransactionTrait;

use super::provider;
use crate::{
    common::{enums::MediaType, error::Result, state::AppState},
    entity::{
        task,
        tv::{self, episode},
    },
};

pub async fn batch_save_episode_download_task(
    state: &AppState,
    tv_info: &tv::Model,
    need_download_resources: Vec<&provider::EpisodeResource>,
) -> Result<()> {
    for r in need_download_resources {
        let txn = state.db.begin().await?;

        let rows_affected = episode::update::update_status_to_queued(
            &txn,
            tv_info.id,
            r.episode.season_number.unwrap(),
            r.episode.episode_number.unwrap(),
        )
        .await?;
        if rows_affected == 0 {
            // 没有变化
            continue;
        }

        task::create::create_download_media_file_task(
            &txn,
            &task::param::DownloadMediaFileParam {
                media_type: MediaType::Tv,
                media_id: tv_info.id,
                year: tv_info.year,
                original_name: tv_info.original_name.to_owned(),
                season_number: r.episode.season_number,
                episode_number: r.episode.episode_number,
                file_type: r.episode.file_type,
                file_ext: r.episode.extension.as_ref().unwrap_or(&"".to_owned()).to_owned(),
                file_url: r.file_url.to_owned(),
                file_downloader: r.file_downloader,
            },
        )
        .await?;

        txn.commit().await?;
    }
    Ok(())
}
