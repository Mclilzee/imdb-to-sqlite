use crate::{config::Args, utils::{parse_sqlite_err, percentage_printer}};
use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

struct TitleDirectors {
    title_id: u32,
    name_ids: Vec<u32>,
}

impl TitleDirectors {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id = values
            .first()
            .and_then(|&s| s.get(2..))
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(format!("Failed to parse title_id from {line}"))?;

        let directors = values
            .get(1)
            .map(|&s| s.split(','))
            .map(|s| {
                s.filter_map(|v| v.get(2..))
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .ok_or(format!("Failed to parse directores from {line}"))?;

        Ok(Self {
            title_id,
            name_ids: directors,
        })
    }
}

pub async fn parse_title_directors(
    file_name: &str,
    table_name: &str,
    conn: &mut SqliteConnection,
    args: &Args
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

    for (i, title_directors) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleDirectors::from))
        .enumerate()
    {
        let title_directors = title_directors?;
        for name_id in title_directors.name_ids.iter() {
            let query = format!("INSERT INTO {table_name} VALUES($1, $2)");
            let _ = sqlx::query(&query)
                .bind(title_directors.title_id)
                .bind(name_id)
                .execute(&mut *tx)
                .await
                .inspect_err(|e| {
                    if args.log {
                        eprintln!(
                            "Failed to insert {}, {} into {table_name} => {e}",
                            title_directors.title_id, name_id
                        );
                    }
                });
        }
        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn create_table(table_name: &str, conn: &mut SqliteConnection, overwrite: bool) -> Result<(), String> {
    if overwrite {
    sqlx::raw_sql(format!("DROP TABLE {table_name}").as_str())
        .execute(&mut *conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;
    }
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, name_id integer not null, foreign key(title_id) references title(id), foreign key(name_id) references name(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
