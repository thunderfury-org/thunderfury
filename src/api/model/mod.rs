use serde::Serialize;
use utoipa::ToSchema;

pub mod genre;
pub mod library;

#[derive(Debug, Serialize, ToSchema)]
pub struct EmptyResponse {}
