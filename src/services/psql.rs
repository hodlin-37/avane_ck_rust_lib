use anyhow::{Context, Result};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls, Row, types::ToSql};

pub async fn pg_host_client(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    dbname: &str,
    max_connections: usize,
) -> Result<Pool> {
    let mut config = Config::new();
    config
        .host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);

    let mgr = Manager::new(config, NoTls);
    let pool = Pool::builder(mgr).max_size(max_connections).build()?;

    Ok(pool)
}

pub async fn pg_truncate_cascade(pool: &Pool, schema: &str, table: &str) -> Result<()> {
    let client = pool.get().await?;

    let query = format!(r#"TRUNCATE TABLE "{}"."{}" CASCADE"#, schema, table);
    client.batch_execute(&query).await?;

    Ok(())
}

pub async fn pg_insert_rows(
    pool: &Pool,
    schema: &str,
    table: &str,
    columns: Vec<&str>,
    data: Vec<Vec<&(dyn ToSql + Sync)>>,
) -> Result<()> {
    let client = pool
        .get()
        .await
        .context("Veritabanı bağlantısı alınamadı")?;

    let column_list = columns
        .iter()
        .map(|col| format!(r#""{}""#, col))
        .collect::<Vec<_>>()
        .join(", ");

    let placeholders = |row_len: usize, row_index: usize| {
        (0..row_len)
            .map(|i| format!("${}", row_index * row_len + i + 1))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut all_values: Vec<&(dyn ToSql + Sync)> = vec![];

    let values_clause = data
        .iter()
        .enumerate()
        .map(|(i, row)| {
            all_values.extend_from_slice(row);
            format!("({})", placeholders(row.len(), i))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        r#"INSERT INTO "{}"."{}" ({}) VALUES {}"#,
        schema, table, column_list, values_clause
    );

    let affected = client
        .execute(query.as_str(), &all_values)
        .await
        .context("INSERT sorgusu başarısız")?;

    println!("✅ {} satır INSERT edildi", affected);

    Ok(())
}

pub async fn pg_select_with_query(
    pool: &Pool,
    query: &str,
) -> Result<Vec<Row>> {
    let client = pool
        .get()
        .await
        .context("Veritabanı bağlantısı alınamadı")?;

    let rows = client
        .query(query, &[])
        .await
        .context("SELECT sorgusu başarısız")?;

    println!("✅ SELECT sorgusu başarıyla çalıştırıldı");
    Ok(rows)
}