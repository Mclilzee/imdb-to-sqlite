use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use crate::{config::Args, utils::percentage_printer};
use sqlx::{Connection, SqliteConnection};

struct TitleRating {
    title_id: u32,
    average_rating: f32,
    votes: u32,
}

impl TitleRating {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id = values
            .first()
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(format!("Failed to parse title_id from {line}"))?;

        let average_rating = values
            .get(1)
            .and_then(|s| s.parse::<f32>().ok())
            .ok_or(format!("Failed to parse average_rating from {line}"))?;

        let votes = values
            .get(2)
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(format!("Failed to parse votes from {line}"))?;

        Ok(Self {
            title_id,
            average_rating,
            votes,
        })
    }
}

pub async fn parse_title_ratings(
    file_name: &str,
    table_name: &str,
    conn: &mut SqliteConnection,
    args: &Args,
) -> Result<(), String> {
    println!("-- Inserting Into {table_name} --");
    create_table(table_name, conn, args.overwrite).await?;
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

    for (i, title_rating) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleRating::from))
        .enumerate()
    {
        let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3)");
        let title_rating = title_rating?;
        let _ = sqlx::query(&query)
            .bind(title_rating.title_id)
            .bind(title_rating.average_rating)
            .bind(title_rating.votes)
            .execute(&mut *tx)
            .await
            .inspect_err(|e| {
                if args.log {
                    eprintln!(
                        "Failed to insert {}, {}, {} into {table_name} => {e}",
                        title_rating.title_id, title_rating.average_rating, title_rating.votes,
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
        sqlx::raw_sql(format!("DROP TABLE {table_name}").as_str())
            .execute(&mut *conn)
            .await
            .map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;
    }
    sqlx::raw_sql(format!(
            "CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, average_rating real not null, votes integer not null, foreign key(title_id) references title(id))",
        ).as_str())
        .execute(conn)
        .await
        .map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
