use sqlx::PgPool;
use std::env;

#[derive(Debug, Clone)]
pub struct TableConn {
    pub table_name: String,
    pub column_name: String,
}

fn get_column_names() -> Option<TableConn> {
    match env::var("JWT_ORG_CUSTOM_CLAIM") {
        Err(_err) => None,
        Ok(custom_claim) => {
            let split_custom_claim: Vec<&str> = custom_claim.split(".").collect();

            Some(TableConn {
                table_name: split_custom_claim[0].to_string(),
                column_name: split_custom_claim[1].to_string(),
            })
        }
    }
}

pub async fn check_org_column(pg_pool: &PgPool) -> Result<Option<TableConn>, sqlx::Error> {
    match get_column_names() {
        None => Ok(None),
        Some(table_conn) => {
            let org_claim_query = format!(
                "SELECT user_id, {} AS org_id FROM {} LIMIT 1;",
                table_conn.column_name, table_conn.table_name
            );

            sqlx::query(&org_claim_query).execute(pg_pool).await?;

            Ok(Some(table_conn))
        }
    }
}
