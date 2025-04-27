pub mod erc20_transfer_model {
    use chrono::NaiveDateTime;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "erc20_transfer")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: u64,
        pub token: String,
        pub from: String,
        pub to: String,
        pub value: String,
        pub tx_hash: String,
        pub timestamp: u64,
        pub block_number: u64,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
        // SeaORM doesn't include deleted_at by default like GORM
        // Add it manually if needed:
        // pub deleted_at: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod erc721_transfer_model {
    use chrono::NaiveDateTime;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "erc721_transfer")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: u64,
        pub token: String,
        pub from: String,
        pub to: String,
        pub token_id: String,
        pub tx_hash: String,
        pub timestamp: u64,
        pub block_number: u64,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
        // SeaORM doesn't include deleted_at by default like GORM
        // Add it manually if needed:
        // pub deleted_at: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
