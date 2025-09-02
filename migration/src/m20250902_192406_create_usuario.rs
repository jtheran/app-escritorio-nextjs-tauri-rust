use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Usuario::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Usuario::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Usuario::Nombre).string().not_null())
                    .col(ColumnDef::new(Usuario::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(Usuario::Password).string().not_null())
                    .col(
                        ColumnDef::new(Usuario::FechaRegistro)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Usuario::UltimoAcceso).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Usuario::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Usuario {
    Table,
    Id,
    Nombre,
    Email,
    Password,
    FechaRegistro,
    UltimoAcceso,
}
