use actix_web::{get, web, HttpResponse};
use reqwest::header::LOCATION;
use serde::Deserialize;

const TMDB_IMAGE_URL: &str = "https://image.tmdb.org/t/p";

#[derive(Deserialize)]
pub struct GetImageRequest {
    namespace: String,
    resolution: String,
    path: String,
}

#[get("/images/{namespace}/{resolution}/{path}")]
pub async fn get_image(req: web::Path<GetImageRequest>) -> HttpResponse {
    if req.namespace == "m" {
        HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, format!("{}/{}/{}", TMDB_IMAGE_URL, req.resolution, req.path)))
            .finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
