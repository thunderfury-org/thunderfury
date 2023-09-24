use std::collections::{HashMap, HashSet};

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};
use tracing::info;

use super::{download, filter, provider};
use crate::{
    common::{error::Result, state::AppState},
    entity::{subscription, tv},
    utils::tmdb::{self, model::EpisodeDetail},
};

pub async fn run_tv_subscription(state: &AppState, sub: &subscription::Model) -> Result<()> {
    let tv_info = tv::query::get_or_fail(&state.db, sub.media_id).await?;

    let episode_numbers_need_fetch = find_episode_numbers_need_fetch(state, &tv_info, sub.season_number).await?;
    if episode_numbers_need_fetch.is_empty() {
        info!(sub_id = sub.id, "no episode numbers need to fetch");
        return Ok(());
    }

    let mut episode_resources =
        provider::fetch_episode_resources(state, &sub.resource_provider, &sub.resource_url).await?;

    // 修复 season number
    fix_episode_season_number(&tv_info, sub, &mut episode_resources);

    let mut need_download_resources = filter::filter_episode(&episode_resources, sub, &episode_numbers_need_fetch);
    need_download_resources.sort_by(|a, b| {
        let order = a.episode.season_number.cmp(&b.episode.season_number);
        if order != std::cmp::Ordering::Equal {
            order
        } else {
            a.episode.episode_number.cmp(&b.episode.episode_number)
        }
    });
    download::batch_save_episode_download_task(state, &tv_info, need_download_resources).await?;

    Ok(())
}

fn fix_episode_season_number(
    tv_info: &tv::Model,
    sub: &subscription::Model,
    resources: &mut Vec<provider::EpisodeResource>,
) {
    for r in resources {
        if r.episode.season_number.is_none() {
            // 没有解析到 season
            if sub.season_number.is_none() {
                // 订阅没有指定 season
                if tv_info.number_of_seasons == 1 {
                    // 只有一个季
                    r.episode.season_number = Some(1);
                } else {
                    // 多个季
                    continue;
                }
            } else {
                // 订阅指定了 season
                r.episode.season_number = Some(sub.season_number.unwrap());
            }
        }
    }
}

async fn find_episode_numbers_need_fetch(
    state: &AppState,
    tv_info: &tv::Model,
    season_number: Option<i32>,
) -> Result<HashMap<i32, HashSet<i32>>> {
    let seasons = get_or_create_seasons(state, tv_info, season_number).await?;

    let episodes: Vec<(i32, i32)> = episode::Entity::find()
        .select_only()
        .column(episode::Column::SeasonNumber)
        .column(episode::Column::EpisodeNumber)
        .filter(episode::Column::TvId.eq(tv_info.id))
        .filter(episode::Column::SeasonNumber.is_in(seasons.iter().map(|s| s.season_number)))
        .filter(episode::Column::Status.eq(episode::Status::Waiting))
        .into_tuple()
        .all(&state.db)
        .await?;

    Ok(episodes.iter().fold(HashMap::new(), |mut m, e| {
        m.entry(e.0).or_default().insert(e.1);
        m
    }))
}

async fn get_or_create_seasons(
    state: &AppState,
    tv_info: &tv::Model,
    season_number: Option<i32>,
) -> Result<Vec<season::Model>> {
    let mut exists_seasons = get_all_exists_seasons(&state.db, tv_info.id, season_number).await?;
    let exists_season_numbers: HashSet<i32> = exists_seasons.iter().map(|s| s.season_number).collect();

    match season_number {
        Some(season_number) => {
            if !exists_season_numbers.contains(&season_number) {
                exists_seasons.push(create_season(state, tv_info.id, tv_info.tmdb_id, season_number).await?);
            }
        }
        None => {
            for s in 1..=tv_info.number_of_seasons {
                if !exists_season_numbers.contains(&s) {
                    exists_seasons.push(create_season(state, tv_info.id, tv_info.tmdb_id, s).await?);
                }
            }
        }
    }

    Ok(exists_seasons)
}

async fn get_all_exists_seasons(
    db: &DatabaseConnection,
    tv_id: i32,
    season_number: Option<i32>,
) -> Result<Vec<season::Model>> {
    match season_number {
        Some(season_number) => Ok(season::Entity::find()
            .filter(season::Column::TvId.eq(tv_id))
            .filter(season::Column::SeasonNumber.eq(season_number))
            .all(db)
            .await?),
        None => Ok(season::Entity::find()
            .filter(season::Column::TvId.eq(tv_id))
            .all(db)
            .await?),
    }
}

async fn create_season(state: &AppState, tv_id: i32, tmdb_id: i32, season_number: i32) -> Result<season::Model> {
    info!("create season {} of tv {}", season_number, tv_id);

    let season_detail = tmdb::Client::try_from(state)?
        .get_season_detail(tmdb_id, season_number)
        .await?;

    let txn = state.db.begin().await?;

    batch_create_episodes(&txn, tv_id, season_number, &season_detail.episodes).await?;

    let new_season = season::ActiveModel {
        tv_id: Set(tv_id),
        season_number: Set(season_number),
        air_date: Set(season_detail.air_date),
        number_of_episodes: Set(season_detail.episodes.len().try_into().unwrap()),
        overview: Set(season_detail.overview),
        poster_path: Set(season_detail.poster_path.to_owned()),
        create_time: Set(Utc::now()),
        ..Default::default()
    };

    let created = new_season.insert(&txn).await?;
    txn.commit().await?;

    Ok(created)
}

async fn batch_create_episodes(
    db: &DatabaseTransaction,
    tv_id: i32,
    season_number: i32,
    episodes: &[EpisodeDetail],
) -> Result<()> {
    episode::Entity::insert_many(episodes.iter().map(|e| episode::ActiveModel {
        tv_id: Set(tv_id),
        season_number: Set(season_number),
        episode_number: Set(e.episode_number),
        name: Set(e.name.to_owned()),
        air_date: Set(e.air_date.to_owned()),
        status: Set(episode::Status::Waiting),
        overview: Set(e.overview.to_owned()),
        still_path: Set(e.still_path.to_owned()),
        create_time: Set(Utc::now()),
        ..Default::default()
    }))
    .exec(db)
    .await?;

    Ok(())
}
