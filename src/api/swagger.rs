use utoipa::OpenApi;

use super::{library, model, provider, subscription};

#[derive(OpenApi)]
#[openapi(
    paths(
        library::list_tvs,
        library::list_movies,
        subscription::query::list_subscriptions,
        subscription::create::new_subscription,
        subscription::run::run_subscription,
        provider::alist::list_files,
        provider::tmdb::search_tv,
    ),
    components(schemas(
        model::library::TvDetail,
        model::library::MovieDetail,
        model::genre::Genre,
        subscription::SubscriptionDetail,
        subscription::NewSubscriptionRequest,
        model::EmptyResponse,
        provider::alist::File,
        provider::tmdb::MediaInfo,
    ))
)]
pub struct ApiDoc;
