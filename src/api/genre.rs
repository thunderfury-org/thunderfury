use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::genre::Genre,
    },
    common::state::AppState,
    entity::genre,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<Genre>),
    )
)]
#[get("/genres")]
pub async fn list_genres(state: web::Data<AppState>) -> ApiResult<Vec<Genre>> {
    ok(genre::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

impl From<genre::Model> for Genre {
    fn from(val: genre::Model) -> Self {
        Genre {
            id: val.id,
            name: val.name,
        }
    }
}
