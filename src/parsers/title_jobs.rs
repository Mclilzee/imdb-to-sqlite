use crate::{config::Args, utils::percentage_printer};
use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

pub struct TitlePrincipal {
    title_id: u32,
    name_id: u32,
    category: String,
    job: Option<String>,
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
            .filter(|&s| *s != "\\N")
            .map(|&s| s.to_string().to_lowercase())
            .ok_or(format!("Failed to parse category from {line}"))?;

        let job = values
            .get(4)
            .filter(|&s| *s != "\\N")
            .map(|&s| s.to_string().to_lowercase());

        Ok(Self {
            title_id,
            name_id,
            category,
            job,
        })
    }
}

pub async fn parse_title_jobs(
    file_name: &str,
    table_name: &str,
    conn: &mut SqliteConnection,
    args: &Args,
) -> Result<(), String> {
    create_table(table_name, conn, args.overwrite).await?;
    let file =
        File::open(file_name).map_err(|e| format!("Unable to read from {file_name} -> {e}"))?;
    let mut reader = BufReader::new(file);
    let count = (&mut reader).lines().skip(1).count();
    println!("-- Inserting {count} entries into {table_name} --");

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
        let _ = sqlx::query(&query)
            .bind(title_principals.title_id)
            .bind(title_principals.name_id)
            .bind(&title_principals.category)
            .bind(&title_principals.job)
            .execute(&mut *tx)
            .await
            .inspect_err(|e| {
                if args.log {
                    eprintln!(
                        "\nFailed to insert {}, {}, {}, {:?} into {table_name} => {e}",
                        title_principals.title_id,
                        title_principals.name_id,
                        title_principals.category,
                        title_principals.job
                    );
                }
            });

        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn create_table(
    table_name: &str,
    conn: &mut SqliteConnection,
    overwrite: bool,
) -> Result<(), String> {
    if overwrite {
        let _ = sqlx::raw_sql(format!("DROP TABLE {table_name}").as_str())
            .execute(&mut *conn)
            .await;
    }

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, name_id integer not null, category text not null, job text, foreign key(title_id) references title(id), foreign key(name_id) references name(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
