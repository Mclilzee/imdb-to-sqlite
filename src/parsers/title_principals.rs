use crate::utils::percentage_printer;
use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

pub struct TitlePrincipal {
    title_id: u32,
    name_id: u32,
    category: String,
    job: String,
}

impl TitlePrincipal {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id: u32 = values
            .first()
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse title_id from {line}"))?;

        let name_id = values
            .get(2)
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse name_id from {line}"))?;

        let category = values
            .get(3)
            .map(|&s| s.to_string().to_lowercase())
            .ok_or(format!("Failed to parse category from {line}"))?;

        let job = values
            .get(3)
            .map(|&s| s.to_string().to_lowercase())
            .ok_or(format!("Failed to parse original_name from {line}"))?;

        Ok(Self {
            title_id,
            name_id,
            category,
            job,
        })
    }
}

pub async fn parse_title_principals(
    file_name: &str,
    table_name: &str,
    conn: &mut SqliteConnection,
) -> Result<(), String> {
    println!("-- Inserting Into {table_name} --");
    create_table(table_name, conn).await?;
    let file =
        File::open(file_name).map_err(|e| format!("Unable to read from {file_name} -> {e}"))?;
    let mut reader = BufReader::new(file);
    let count = (&mut reader).lines().skip(1).count();
    reader
        .rewind()
        .map_err(|e| format!("Failed to read file {file_name} after counting => {e}"))?;

    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    for (i, title_principals) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitlePrincipal::from))
        .enumerate()
    {
        let title_principals = title_principals?;
        let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3, $4)");
        sqlx::query(&query)
            .bind(title_principals.title_id)
            .bind(title_principals.name_id)
            .bind(&title_principals.category)
            .bind(&title_principals.job)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {}, {} into {table_name} => {e}",
                    title_principals.title_id,
                    title_principals.name_id,
                    title_principals.category,
                    title_principals.job
                )
            })?;

        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn create_table(table_name: &str, conn: &mut SqliteConnection) -> Result<(), String> {
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, name_id integer not null, category text, job text, foreign key(title_id) references title(id), foreign key(name_id) references name(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
