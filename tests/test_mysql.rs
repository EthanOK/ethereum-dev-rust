mod common;

use chrono::Local;
use common::database_connection;
use ethereum_dev::erc20_transfer_model::{
    ActiveModel as ERC20TransferActiveModel, Column as ERC20TransferColumn,
    Entity as ERC20TransferEntity,
};
use eyre::Result;
use sea_orm::{ActiveValue, DatabaseConnection};
pub use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

async fn get_last_id(db: DatabaseConnection) -> Result<u64> {
    let rs =
        ERC20TransferEntity::find().order_by_desc(ERC20TransferColumn::Id).one(&db).await?.unwrap();
    Ok(rs.id)
}

#[tokio::test]
async fn test_read_mysql_data() -> Result<()> {
    let db = database_connection::get_mysql_connection().await?;
    let id: u64 = 1;
    let rs = ERC20TransferEntity::find_by_id(id).one(&db).await?.unwrap();
    println!("find by id(1):");
    println!("{:?}", rs);

    let token = "0xe4a5026a4888e5b9D2587aD3Bd1BD1Cc4Aca0b2d";
    let rs = ERC20TransferEntity::find()
        .filter(ERC20TransferColumn::Token.clone().eq(token))
        .all(&db)
        .await?;
    println!("find by token({}):", token);
    for item in rs.iter() {
        println!("{:?}", item);
    }

    Ok(())
}

#[tokio::test]
async fn test_insert_and_delete_mysql_data() -> Result<()> {
    let db = database_connection::get_mysql_connection().await?;

    let id: u64 = 1;
    let rs = ERC20TransferEntity::find_by_id(id).one(&db).await?.unwrap();
    let timestamp = chrono::Utc::now().timestamp() as u64;
    let created_at = chrono::Utc::now().naive_utc();

    let active_value = ERC20TransferActiveModel {
        id: ActiveValue::NotSet,
        token: ActiveValue::Set(rs.token.clone()),
        from: ActiveValue::Set(rs.from.clone()),
        to: ActiveValue::Set(rs.to.clone()),
        value: ActiveValue::Set(rs.value.clone()),
        timestamp: ActiveValue::Set(timestamp),
        block_number: ActiveValue::Set(rs.block_number.clone()),
        tx_hash: ActiveValue::Set(rs.tx_hash.clone()),
        index: ActiveValue::Set(rs.index.clone()),
        created_at: ActiveValue::Set(Some(created_at)),
        updated_at: ActiveValue::Set(Some(created_at)),
    };

    // insert data
    let rusult = ERC20TransferEntity::insert(active_value).exec(&db).await?;
    println!("last_insert_id: {:?}", rusult.last_insert_id);

    let id: u64 = get_last_id(db.clone()).await?;
    println!("last id: {}", id);

    // delete data
    let rusult = ERC20TransferEntity::delete_by_id(id).exec(&db).await?;
    println!("delete result: {:?}", rusult);

    Ok(())
}

#[tokio::test]
async fn test_date_time() -> Result<()> {
    let created_at = Local::now().naive_local();
    println!("created_at: {:?}", created_at);
    Ok(())
}
