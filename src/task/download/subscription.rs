use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use crate::{
    common::error::Result,
    entity::{enums::MediaType, episode, season, subscription},
    task::param::DownloadMediaFileParam,
};

pub async fn mark_subscription_done_if_complete<C>(db: &C, task_param: &DownloadMediaFileParam) -> Result<()>
where
    C: ConnectionTrait,
{
    match task_param.media_type {
        MediaType::Tv => mark_tv_subscription_done(db, task_param).await,
        MediaType::Movie => todo!("support movie subscription"),
    }
}

async fn mark_tv_subscription_done<C>(db: &C, task_param: &DownloadMediaFileParam) -> Result<()>
where
    C: ConnectionTrait,
{
    let tv_id = task_param.media_id;
    let season_number = task_param.season_number.unwrap();

    let sub_ids = get_all_running_tv_subscription_ids(db, tv_id, season_number).await?;
    if sub_ids.is_empty() {
        return Ok(());
    }

    if !is_all_episodes_downloaded(db, tv_id, season_number).await? {
        return Ok(());
    }

    mark_subscriptions_done(db, sub_ids).await
}

async fn get_all_running_tv_subscription_ids<C>(db: &C, tv_id: i32, season_number: i32) -> Result<Vec<i32>>
where
    C: ConnectionTrait,
{
    Ok(subscription::Entity::find()
        .select_only()
        .column(subscription::Column::Id)
        .filter(subscription::Column::MediaType.eq(MediaType::Tv))
        .filter(subscription::Column::MediaId.eq(tv_id))
        .filter(subscription::Column::SeasonNumber.eq(season_number))
        .filter(subscription::Column::Status.eq(subscription::Status::Running))
        .into_tuple()
        .all(db)
        .await?)
}

async fn is_all_episodes_downloaded<C>(db: &C, tv_id: i32, season_number: i32) -> Result<bool>
where
    C: ConnectionTrait,
{
    let downloaded_episode_count = episode::Entity::find()
        .filter(episode::Column::TvId.eq(tv_id))
        .filter(episode::Column::SeasonNumber.eq(season_number))
        .filter(episode::Column::Status.eq(episode::Status::Downloaded))
        .count(db)
        .await?;

    let season_episode_count: i32 = season::Entity::find()
        .select_only()
        .column(season::Column::NumberOfEpisodes)
        .filter(season::Column::TvId.eq(tv_id))
        .filter(season::Column::SeasonNumber.eq(season_number))
        .into_tuple()
        .one(db)
        .await?
        .unwrap();

    Ok((season_episode_count as u64) == downloaded_episode_count)
}

async fn mark_subscriptions_done<C>(db: &C, sub_ids: Vec<i32>) -> Result<()>
where
    C: ConnectionTrait,
{
    subscription::Entity::update_many()
        .set(subscription::ActiveModel {
            status: Set(subscription::Status::Done),
            finish_time: Set(Some(Utc::now())),
            ..Default::default()
        })
        .filter(subscription::Column::Id.is_in(sub_ids))
        .filter(subscription::Column::Status.eq(subscription::Status::Running))
        .exec(db)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

    use crate::{
        common::error::Result,
        entity::{
            enums::{MediaType, Provider},
            episode, season, subscription, tv,
        },
        task::param::DownloadMediaFileParam,
        test,
    };

    #[tokio::test]
    async fn mark_tv_subscription_done() -> Result<()> {
        let db = test::init_test_db().await;

        let tv_id = prepare_tv_data(&db).await?;

        // episode not downloaded
        assert!(!super::is_all_episodes_downloaded(&db, tv_id, 1).await?);

        // all episodes downloaded
        mark_all_episode_downloaded(&db, tv_id, 1).await?;
        assert!(super::is_all_episodes_downloaded(&db, tv_id, 1).await?);

        super::mark_subscription_done_if_complete(
            &db,
            &DownloadMediaFileParam {
                media_type: MediaType::Tv,
                media_id: tv_id,
                season_number: Some(1),
                ..Default::default()
            },
        )
        .await?;

        let sub = subscription::Entity::find().one(&db).await?.unwrap();
        assert_eq!(sub.status, subscription::Status::Done);
        assert!(sub.finish_time.is_some());

        Ok(())
    }

    async fn prepare_tv_data<C>(db: &C) -> Result<i32>
    where
        C: ConnectionTrait,
    {
        let tv_id = tv::Entity::insert(tv::ActiveModel {
            name: Set("test".to_owned()),
            year: Set(2022),
            status: Set("Ended".to_owned()),
            first_air_date: Set("2022-01-01".to_owned()),
            number_of_seasons: Set(1),
            tmdb_id: Set(1),
            original_language: Set("en".to_owned()),
            original_name: Set("test".to_owned()),
            overview: Set("test".to_owned()),
            poster_path: Set("".to_owned()),
            backdrop_path: Set("".to_owned()),
            create_time: Set(Utc::now()),
            ..Default::default()
        })
        .exec(db)
        .await?
        .last_insert_id;

        season::Entity::insert(season::ActiveModel {
            tv_id: Set(tv_id),
            season_number: Set(1),
            air_date: Set("2022-01-01".to_owned()),
            number_of_episodes: Set(2),
            overview: Set("test".to_owned()),
            poster_path: Set("".to_string()),
            create_time: Set(Utc::now()),
            ..Default::default()
        })
        .exec(db)
        .await?;

        for i in 1..=2 {
            episode::Entity::insert(episode::ActiveModel {
                tv_id: Set(tv_id),
                season_number: Set(1),
                episode_number: Set(i),
                name: Set("test".to_owned()),
                air_date: Set("2022-01-01".to_owned()),
                status: Set(episode::Status::Waiting),
                overview: Set("test".to_owned()),
                still_path: Set("".to_owned()),
                create_time: Set(Utc::now()),
                ..Default::default()
            })
            .exec(db)
            .await?;
        }

        subscription::Entity::insert(subscription::ActiveModel {
            unique_id: Set("test".to_owned()),
            media_type: Set(MediaType::Tv),
            media_id: Set(tv_id),
            tmdb_id: Set(1),
            resource_provider: Set(Provider::Rss),
            resource_url: Set(Some("test".to_owned())),
            season_number: Set(Some(1)),
            status: Set(subscription::Status::Running),
            next_run_time: Set(Utc::now()),
            create_time: Set(Utc::now()),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Ok(tv_id)
    }

    async fn mark_all_episode_downloaded<C>(db: &C, tv_id: i32, season_number: i32) -> Result<()>
    where
        C: ConnectionTrait,
    {
        episode::Entity::update_many()
            .set(episode::ActiveModel {
                status: Set(episode::Status::Downloaded),
                ..Default::default()
            })
            .filter(episode::Column::TvId.eq(tv_id))
            .filter(episode::Column::SeasonNumber.eq(season_number))
            .exec(db)
            .await?;

        Ok(())
    }
}
