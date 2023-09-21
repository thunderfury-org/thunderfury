use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use super::query;
use crate::{
    common::{
        error::{Error, Result},
        state::AppState,
    },
    entity::{enums::MediaType, subscription, tv},
    utils::tmdb::{self, model::TvDetail},
};

pub async fn create_subscription(state: &AppState, request: &subscription::Model) -> Result<subscription::Model> {
    if let Some(exist_sub) = query::get_subscription_by_unique_id(&state.db, request.unique_id()).await? {
        return Ok(exist_sub);
    }

    let txn = state.db.begin().await?;

    let media_id = match request.media_type {
        MediaType::Tv => {
            let new_tv = create_tv(state, &txn, request.tmdb_id).await?;
            if let Some(s) = request.season_number {
                if s > new_tv.number_of_seasons {
                    return Err(Error::InvalidArgument(format!(
                        "season number must be less than or equal to {}",
                        new_tv.number_of_seasons
                    )));
                }
            }

            new_tv.id
        }
        MediaType::Movie => todo!(),
    };

    let sub = save_subscription(&txn, request, media_id).await?;
    txn.commit().await?;

    Ok(sub)
}

async fn save_subscription(
    txn: &DatabaseTransaction,
    request: &subscription::Model,
    media_id: u32,
) -> Result<subscription::Model> {
    let new_sub = subscription::ActiveModel {
        unique_id: Set(request.unique_id()),
        media_type: Set(request.media_type.to_owned()),
        media_id: Set(media_id),
        tmdb_id: Set(request.tmdb_id),
        resource_provider: Set(request.resource_provider.to_owned()),
        resource_url: Set(request.resource_url.to_owned()),
        season_number: Set(request.season_number),
        resolutions: Set(request.resolutions.to_owned()),
        subtitles: Set(request.subtitles.to_owned()),
        status: Set(subscription::Status::Running),
        last_run_time: NotSet,
        next_run_time: Set(Utc::now()),
        create_time: Set(Utc::now()),
        ..Default::default()
    };

    Ok(new_sub.insert(txn).await?)
}

async fn create_tv(state: &AppState, txn: &DatabaseTransaction, tmdb_id: u32) -> Result<tv::Model> {
    if let Some(exists) = tv::Entity::find()
        .filter(tv::Column::TmdbId.eq(tmdb_id))
        .one(txn)
        .await?
    {
        return Ok(exists);
    }

    // not exists, create new tv
    let detail = tmdb::Client::try_from(state)?.get_tv_detail(tmdb_id).await?;
    save_tv(txn, &detail).await
}

async fn save_tv(txn: &DatabaseTransaction, detail: &TvDetail) -> Result<tv::Model> {
    let year: u32 = detail.first_air_date.split('-').next().unwrap_or("0").parse().unwrap();

    let new_tv = tv::ActiveModel {
        name: Set(detail.name.to_owned()),
        year: Set(year),
        status: Set(detail.status.to_owned()),
        first_air_date: Set(detail.first_air_date.to_owned()),
        number_of_seasons: Set(detail.number_of_seasons),
        tmdb_id: Set(detail.id),
        original_language: Set(detail.original_language.to_owned()),
        original_name: Set(detail.original_name.to_owned()),
        overview: Set(detail.overview.to_owned()),
        poster_path: Set(detail.poster_path.to_owned()),
        backdrop_path: Set(detail.backdrop_path.to_owned()),
        create_time: Set(Utc::now()),
        ..Default::default()
    };

    Ok(new_tv.insert(txn).await?)
}
