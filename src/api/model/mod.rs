use serde::Serialize;
use utoipa::ToSchema;

pub mod genre;
pub mod library;
pub mod subscription;

#[derive(Debug, Serialize, ToSchema)]
pub struct EmptyResponse {}
