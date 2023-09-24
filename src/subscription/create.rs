use sea_orm::{DatabaseTransaction, TransactionTrait};

use crate::{
    common::{
        enums::MediaType,
        error::{Error, Result},
        state::AppState,
    },
    entity::{subscription, tv},
    utils::tmdb,
};

pub async fn create_subscription(state: &AppState, request: &subscription::Model) -> Result<subscription::Model> {
    if let Some(exist_sub) = subscription::query::get_by_unique_id(&state.db, request.unique_id().as_str()).await? {
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

    let sub = subscription::create::create_subscription(
        &txn,
        subscription::create::NewSubscription {
            unique_id: request.unique_id(),
            media_type: request.media_type,
            media_id,
            tmdb_id: request.tmdb_id,
            resource_provider: request.resource_provider,
            resource_url: request.resource_url.clone(),
            season_number: request.season_number,
            resolutions: request.resolutions.clone(),
            subtitles: request.subtitles.clone(),
        },
    )
    .await?;
    txn.commit().await?;

    Ok(sub)
}

async fn create_tv(state: &AppState, txn: &DatabaseTransaction, tmdb_id: i32) -> Result<tv::Model> {
    if let Some(exists) = tv::query::get_by_tmdb_id(txn, tmdb_id).await? {
        return Ok(exists);
    }

    // not exists, create new tv
    let detail = tmdb::Client::try_from(state)?.get_tv_detail(tmdb_id).await?;

    let year: i32 = detail.first_air_date.split('-').next().unwrap_or("0").parse().unwrap();
    tv::create::create_tv(
        txn,
        tv::create::NewTv {
            name: detail.name,
            year,
            status: detail.status,
            first_air_date: detail.first_air_date,
            number_of_seasons: detail.number_of_seasons,
            tmdb_id,
            original_language: detail.original_language,
            original_name: detail.original_name,
            overview: detail.overview,
            poster_path: detail.poster_path,
            backdrop_path: detail.backdrop_path,
        },
    )
    .await
}
