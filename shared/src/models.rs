use serde::{Deserialize, Serialize};

#[derive(
    sqlx::FromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(
    sqlx::FromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: String,
}
