use ethereum_dev::erc20_transfer_model::Entity as ERC20Transfer;
use eyre::Result;
use sea_orm::{Database, EntityTrait};

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "mysql://root:root@localhost:3306/ethereum-dev?charset=utf8mb4";

    let db = Database::connect(db_url).await?;

    let id: u64 = 1000;

    let rs = ERC20Transfer::find_by_id(id).one(&db).await?;
    println!("{:?}", rs);
    Ok(())
}
