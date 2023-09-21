use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}
