use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct OrgTableInfo {
    pub table_name: String,
    pub column_name: String,
}

fn parse_connections(connections: Option<String>) -> u32 {
    connections.map_or(5, |value| value.parse::<u32>().unwrap_or(5))
}

pub async fn connect_and_migrate(db_url: &String, connections: Option<String>) -> Result<PgPool, sqlx::Error> {
    let max_connections = parse_connections(connections);
    let pg_pool = PgPoolOptions::new().max_connections(max_connections).connect(db_url).await?;

    sqlx::migrate!().run(&pg_pool).await?;

    Ok(pg_pool)
}

pub fn get_org_table_info(org_table_column: Option<String>) -> Option<OrgTableInfo> {
    org_table_column.map(|org_string| {
        let split_org_string: Vec<&str> = org_string.split(".").collect();

        OrgTableInfo {
            table_name: split_org_string[0].to_string(),
            column_name: split_org_string[1].to_string(),
        }
    })
}
