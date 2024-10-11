use serde::{Deserialize, Serialize};

mod affiliate_old;
mod affiliate;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NumberedItem<T> {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    #[serde(flatten)]
    pub content: T,
}