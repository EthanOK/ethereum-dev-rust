use crate::erc20_transfer_model::ActiveModel;
use crate::erc20_transfer_model::Entity as ERC20TransferEntity;
use chrono::Local;
use eyre::Result;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

pub async fn handle_erc20_transfer_event(
    mut active_model: ActiveModel,
    db: DatabaseConnection,
) -> Result<()> {
    let created_at = Local::now().naive_local();
    active_model.created_at = ActiveValue::Set(Some(created_at));
    active_model.updated_at = ActiveValue::Set(Some(created_at));

    // insert data
    let _rusult = ERC20TransferEntity::insert(active_model).exec(&db).await?;
    Ok(())
}
