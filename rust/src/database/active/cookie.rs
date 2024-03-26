pub type CookieModel = Model;

use crate::database::active::ACTIVE_DATABASE;
use crate::database::commons::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::IntoActiveModel;
use std::ops::Deref;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cookie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub domain: String,
    #[sea_orm(primary_key)]
    pub path: String,
    #[sea_orm(primary_key)]
    pub name: String,
    pub value: String,
    pub expires: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(db.deref(), "cookie", "idx_cookie_domain").await {
        create_index(db.deref(), "cookie", vec!["domain"], "idx_cookie_domain").await;
    }
}

pub(crate) async fn update_cookies(cookies: &Vec<CookieModel>) -> Result<(), DbErr> {
    let lock = ACTIVE_DATABASE.get().unwrap().lock().await;
    for cookie in cookies {
        Entity::insert(cookie.clone().into_active_model())
            .on_conflict(
                OnConflict::columns(vec![Column::Domain, Column::Path, Column::Name])
                    .update_columns(vec![Column::Value, Column::Expires])
                    .to_owned(),
            )
            .exec(lock.deref())
            .await?;
    }
    Ok(())
}

pub(crate) async fn cookies() -> Result<Vec<Model>, DbErr> {
    let lock = ACTIVE_DATABASE.get().unwrap().lock().await;
    Entity::find().all(lock.deref()).await
}
