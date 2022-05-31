use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct TableConn {
    pub table_name: String,
    pub column_name: String,
}

pub async fn connect_and_migrate(db_url: &String) -> Result<PgPool, sqlx::Error> {
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::migrate!().run(&pg_pool).await?;

    Ok(pg_pool)
}

pub async fn check_org_column(
    pg_pool: &PgPool,
    custom_claim: Option<String>,
) -> Result<Option<TableConn>, sqlx::Error> {
    match custom_claim {
        None => Ok(None),
        Some(custom_claim) => {
            let split_custom_claim: Vec<&str> = custom_claim.split(".").collect();

            let table_conn = TableConn {
                table_name: split_custom_claim[0].to_string(),
                column_name: split_custom_claim[1].to_string(),
            };

            let org_claim_query = format!(
                "SELECT user_id, {} AS org_id FROM {} LIMIT 1;",
                table_conn.column_name, table_conn.table_name
            );

            sqlx::query(&org_claim_query).execute(pg_pool).await?;

            Ok(Some(table_conn))
        }
    }
}
