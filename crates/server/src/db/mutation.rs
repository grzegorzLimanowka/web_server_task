use ::entity::{prelude::*, requests};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_request(
        db: &DbConn,
        data: requests::Model,
    ) -> Result<requests::ActiveModel, DbErr> {
        requests::ActiveModel {
            batch_id: Set(data.batch_id),
            value: Set(data.value),
            status: Set(data.status),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_request(
        db: &DbConn,
        id: i32,
        data: requests::Model,
    ) -> Result<requests::Model, DbErr> {
        let request: requests::ActiveModel = Requests::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find request".to_owned()))
            .map(Into::into)?;

        requests::ActiveModel {
            id: Set(id),
            batch_id: Set(data.batch_id),
            value: Set(data.value),
            status: Set(data.status),
        }
        .update(db)
        .await
    }

    pub async fn delete_request(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Requests::delete_many().exec(db).await
    }
}
