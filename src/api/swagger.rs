use utoipa::OpenApi;

use super::{library, model, provider, subscription};

#[derive(OpenApi)]
#[openapi(
    paths(
        library::list_tvs,
        library::list_movies,
        subscription::list_subscriptions,
        subscription::new_subscription,
        subscription::run_subscription,
        provider::alist::list_files,
        provider::tmdb::search_tv,
    ),
    components(schemas(
        model::library::TvDetail,
        model::library::MovieDetail,
        model::genre::Genre,
        model::subscription::SubscriptionDetail,
        model::subscription::NewSubscriptionRequest,
        model::EmptyResponse,
        provider::alist::File,
        provider::tmdb::MediaInfo,
    ))
)]
pub struct ApiDoc;
