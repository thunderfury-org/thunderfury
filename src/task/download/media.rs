use chrono::Utc;
use sea_orm::{DatabaseTransaction, Set};

use crate::common::error::Result;

pub async fn update_status_to_downloading(
    db: &DatabaseTransaction,
    task_param: &param::DownloadMediaFileParam,
    gid: &str,
) -> Result<()> {
    match task_param.media_type {
        enums::MediaType::Tv => {
            episode::Entity::update_many()
                .set(episode::ActiveModel {
                    status: Set(episode::Status::Downloading),
                    external_task_id: Set(Some(gid.to_owned())),
                    ..Default::default()
                })
                .filter(episode::Column::TvId.eq(task_param.media_id))
                .filter(episode::Column::SeasonNumber.eq(task_param.season_number.unwrap()))
                .filter(episode::Column::EpisodeNumber.eq(task_param.episode_number.unwrap()))
                .filter(episode::Column::Status.eq(episode::Status::Queued))
                .exec(db)
                .await?;
        }
        enums::MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_external_task_id(
    db: &DatabaseTransaction,
    task_param: &param::DownloadMediaFileParam,
    gid: &str,
) -> Result<()> {
    match task_param.media_type {
        enums::MediaType::Tv => {
            episode::Entity::update_many()
                .set(episode::ActiveModel {
                    external_task_id: Set(Some(gid.to_owned())),
                    ..Default::default()
                })
                .filter(episode::Column::TvId.eq(task_param.media_id))
                .filter(episode::Column::SeasonNumber.eq(task_param.season_number.unwrap()))
                .filter(episode::Column::EpisodeNumber.eq(task_param.episode_number.unwrap()))
                .filter(episode::Column::Status.eq(episode::Status::Downloading))
                .exec(db)
                .await?;
        }
        enums::MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_status_to_downloaded(
    db: &DatabaseTransaction,
    task_param: &param::DownloadMediaFileParam,
) -> Result<()> {
    match task_param.media_type {
        enums::MediaType::Tv => {
            episode::Entity::update_many()
                .set(episode::ActiveModel {
                    status: Set(episode::Status::Downloaded),
                    download_time: Set(Some(Utc::now())),
                    file_path: Set(Some(task_param.get_library_file_path())),
                    ..Default::default()
                })
                .filter(episode::Column::TvId.eq(task_param.media_id))
                .filter(episode::Column::SeasonNumber.eq(task_param.season_number.unwrap()))
                .filter(episode::Column::EpisodeNumber.eq(task_param.episode_number.unwrap()))
                .filter(episode::Column::Status.eq(episode::Status::Downloading))
                .exec(db)
                .await?;
        }
        enums::MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_status_to_waiting(
    db: &DatabaseTransaction,
    task_param: &param::DownloadMediaFileParam,
) -> Result<()> {
    match task_param.media_type {
        enums::MediaType::Tv => {
            episode::Entity::update_many()
                .set(episode::ActiveModel {
                    status: Set(episode::Status::Waiting),
                    ..Default::default()
                })
                .filter(episode::Column::TvId.eq(task_param.media_id))
                .filter(episode::Column::SeasonNumber.eq(task_param.season_number.unwrap()))
                .filter(episode::Column::EpisodeNumber.eq(task_param.episode_number.unwrap()))
                .filter(episode::Column::Status.eq(episode::Status::Downloading))
                .exec(db)
                .await?;
        }
        enums::MediaType::Movie => todo!(),
    }

    Ok(())
}
