use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

const AFFILIATE_STATICS_TABLE_NAME: &str = "affiliate.statics";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffiliateStatics{
    pub user_id: Uuid,
    pub invite_code: String,
    pub total: f32,
    pub withdrawn: f32,
    pub count_referrals: i32,
    pub invited_by: Option<Uuid>,
    pub rate: f32,
}

impl AffiliateStatics {
    async fn init(pool: &PgPool, user: Uuid) -> Result<Self, sqlx::Error> {
        todo!()
    }
    async fn update_invite_code(pool: &PgPool, user: Uuid, invite_code: String) -> Result<(), sqlx::Error> {
        todo!()
    }
    async fn find_by_invite_code(pool: &PgPool, invite_code: &str) -> Result<Option<AffiliateStatics>, sqlx::Error> {
        todo!()
    }
}