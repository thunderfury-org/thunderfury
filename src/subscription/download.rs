use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};

use super::provider;
use crate::{
    common::error::Result,
    entity::{enums, episode, tv},
    task,
};

pub async fn batch_save_episode_download_task(
    db: &DatabaseConnection,
    tv_info: &tv::Model,
    need_download_resources: Vec<&provider::EpisodeResource>,
) -> Result<()> {
    for r in need_download_resources {
        let txn = db.begin().await?;

        let result = episode::Entity::update_many()
            .set(episode::ActiveModel {
                status: Set(episode::Status::Queued),
                ..Default::default()
            })
            .filter(episode::Column::TvId.eq(tv_info.id))
            .filter(episode::Column::SeasonNumber.eq(r.episode.season_number.unwrap()))
            .filter(episode::Column::EpisodeNumber.eq(r.episode.episode_number.unwrap()))
            .filter(episode::Column::Status.eq(episode::Status::Waiting))
            .exec(&txn)
            .await?;
        if result.rows_affected == 0 {
            // 没有变化
            continue;
        }

        task::create::create_download_media_file_task(
            &txn,
            &task::param::DownloadMediaFileParam {
                media_type: enums::MediaType::Tv,
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
