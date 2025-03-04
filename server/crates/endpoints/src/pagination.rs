use anyhow::Error;
use database::paginated_models::deserialise_db_key;
use database::paginated_models::DbKey;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i32>,
    pub last_evaluated_key: Option<String>,
}

impl PaginationParams {
    pub fn decode_last_evaluated_key(&self) -> Result<Option<DbKey>, Error> {
        match &self.last_evaluated_key {
            Some(key) => Ok(Some(deserialise_db_key(key)?)),
            None => Ok(None),
        }
    }
}

#[derive(ToSchema, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub last_evaluated_key: Option<String>,
}
