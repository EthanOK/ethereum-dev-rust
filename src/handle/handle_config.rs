use crate::config_model::Column as ConfigColumn;
use crate::config_model::Entity as ConfigEntity;
use chrono::Local;
use eyre::Result;
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_config_map_value(map_key: &str, db: DatabaseConnection) -> Result<String> {
    let result =
        ConfigEntity::find().filter(ConfigColumn::MapKey.eq(map_key)).one(&db).await?.unwrap();

    Ok(result.map_value)
}

pub async fn update_config_map_value(
    map_key: &str,
    map_value: &str,
    db: DatabaseConnection,
) -> Result<()> {
    let updated_at = Local::now().naive_local();
    let _r = ConfigEntity::update_many()
        .col_expr(ConfigColumn::MapValue, Expr::value(map_value))
        .col_expr(ConfigColumn::UpdatedAt, Expr::value(updated_at))
        .filter(ConfigColumn::MapKey.eq(map_key))
        .exec(&db)
        .await?;
    Ok(())
}
