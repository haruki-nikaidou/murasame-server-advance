use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::NumberedItem;

const AFFILIATE_GRAPH_TABLE_NAME: &str = "affiliate.log";

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct AffiliateLog {
    pub parent: Uuid,
    pub child: Uuid,
    pub reward: f32,
    pub rate: f32,
}

impl AffiliateLog {
    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        todo!()
    }
    async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<NumberedItem<String>>, sqlx::Error> {
        todo!()
    }
}