use ::entity::{
    prelude::*,
    requests::{self, ActiveModel},
};
use sea_orm::*;

pub struct Query {}

impl Query {
    pub async fn find_request_by_id(
        id: i32,
        db: &DbConn,
    ) -> Result<Option<requests::Model>, DbErr> {
        Requests::find_by_id(id).one(db).await
    }

    pub async fn find_requests_by_batch_id(
        batch_id: i32,
        db: &DbConn,
    ) -> Result<Vec<requests::Model>, DbErr> {
        Requests::find_by_id(batch_id).all(db).await
    }
}
