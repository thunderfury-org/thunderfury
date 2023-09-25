use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod image;
mod library;
mod model;
mod provider;
mod subscription;
mod swagger;

pub fn api(cfg: &mut web::ServiceConfig) {
    let apis = web::scope("/api")
        .service(library::list_tvs)
        .service(library::list_movies)
        .service(subscription::query::list_subscriptions)
        .service(subscription::create::new_subscription)
        .service(subscription::run::run_subscription)
        .service(provider::alist::list_files)
        .service(provider::tmdb::search_tv);

    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/swagger.json", swagger::ApiDoc::openapi()))
        .service(image::get_image)
        .service(apis);
}
