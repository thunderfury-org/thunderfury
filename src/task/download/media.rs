use sea_orm::DatabaseTransaction;

use crate::{
    common::{enums::MediaType, error::Result},
    entity::{task::param::DownloadMediaFileParam, tv::episode},
};

pub async fn update_status_to_downloading(
    db: &DatabaseTransaction,
    task_param: &DownloadMediaFileParam,
    gid: &str,
) -> Result<()> {
    match task_param.media_type {
        MediaType::Tv => {
            episode::update::update_status_to_downloading(
                db,
                task_param.media_id,
                task_param.season_number.unwrap(),
                task_param.episode_number.unwrap(),
                gid,
            )
            .await?;
        }
        MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_external_task_id(
    db: &DatabaseTransaction,
    task_param: &DownloadMediaFileParam,
    gid: &str,
) -> Result<()> {
    match task_param.media_type {
        MediaType::Tv => {
            episode::update::update_external_task_id(
                db,
                task_param.media_id,
                task_param.season_number.unwrap(),
                task_param.episode_number.unwrap(),
                gid,
            )
            .await?;
        }
        MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_status_to_downloaded(db: &DatabaseTransaction, task_param: &DownloadMediaFileParam) -> Result<()> {
    match task_param.media_type {
        MediaType::Tv => {
            episode::update::update_status_to_downloaded(
                db,
                task_param.media_id,
                task_param.season_number.unwrap(),
                task_param.episode_number.unwrap(),
                &task_param.get_library_file_path(),
            )
            .await?;
        }
        MediaType::Movie => todo!(),
    }

    Ok(())
}

pub async fn update_status_to_waiting(db: &DatabaseTransaction, task_param: &DownloadMediaFileParam) -> Result<()> {
    match task_param.media_type {
        MediaType::Tv => {
            episode::update::update_status_to_waiting(
                db,
                task_param.media_id,
                task_param.season_number.unwrap(),
                task_param.episode_number.unwrap(),
            )
            .await?
        }
        MediaType::Movie => todo!(),
    }

    Ok(())
}
