use utoipa::OpenApi;

use super::{movie, provider, response, subscription, tv};

#[derive(OpenApi)]
#[openapi(
    paths(
        tv::query::list_tvs,
        movie::query::list_movies,
        subscription::query::list_subscriptions,
        subscription::create::new_subscription,
        subscription::run::run_subscription,
        provider::alist::list_files,
        provider::tmdb::search_tv,
    ),
    components(schemas(
        tv::TvDetail,
        movie::MovieDetail,
        subscription::SubscriptionDetail,
        subscription::NewSubscriptionRequest,
        response::EmptyResponse,
        provider::alist::File,
        provider::tmdb::MediaInfo,
    ))
)]
pub struct ApiDoc;
