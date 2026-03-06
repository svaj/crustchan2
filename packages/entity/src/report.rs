use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "post_id", to = "id")]
    pub post: HasOne<super::post::Entity>,

    pub file_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "file_id", to = "id")]
    pub file: HasOne<super::file::Entity>,

    pub user_id: Option<Uuid>, // id of the user who made the post, null if not logged in
    #[sea_orm(belongs_to, from = "user_id", to = "id", relation_enum = "User")]
    pub user: HasOne<super::user::Entity>,

    pub user_identifier_id: Option<Uuid>, // user being banned.
    #[sea_orm(
        belongs_to,
        from = "user_identifier_id",
        to = "id",
        relation_enum = "User_Identifier"
    )]
    pub user_identifier: HasOne<super::user_identifier::Entity>,

    pub reason: String,
    pub reporter_user_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "reporter_user_id",
        to = "id",
        relation_enum = "Reporter"
    )]
    pub reporter_user: HasOne<super::user::Entity>,

    pub reporter_user_identifier_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "reporter_user_identifier_id",
        to = "id",
        relation_enum = "Reporter_User_Identifier"
    )]
    pub reporter_user_identifier: HasOne<super::user_identifier::Entity>,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
