use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use crate::utils::percentage_printer;
use sqlx::{Connection, SqliteConnection};

struct TitleRating {
    title_id: u32,
    average_rating: f32,
    votes: u32,
}

impl TitleRating {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id = values.first().unwrap()[2..]
            .parse::<u32>()
            .map_err(|e| format!("File line ''{line}'' contains wrong format => {e}"))?;

        let average_rating = values.get(1).unwrap().parse::<f32>().unwrap_or_default();

        let votes = values.get(2).unwrap().parse::<u32>().unwrap_or_default();

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

    for (i, title_rating) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleRating::from))
        .enumerate()
    {
        let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3)");
        let title_rating = title_rating?;
        sqlx::query(&query)
            .bind(title_rating.title_id)
            .bind(title_rating.average_rating)
            .bind(title_rating.votes)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {} into {table_name} => {e}",
                    title_rating.title_id, title_rating.average_rating, title_rating.votes,
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
    let query = format!(
            "CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, average_rating real not null, votes integer not null, foreign key(title_id) references title(id))",
        );

    sqlx::raw_sql(&query)
        .execute(conn)
        .await
        .map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
