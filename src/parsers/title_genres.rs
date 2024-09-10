use std::{fs::File, io::{BufRead, BufReader, Seek}};
use sqlx::{SqliteConnection, Connection};
use crate::utils::percentage_printer;

struct TitleGenres {
    title_id: u32,
    genres: Vec<String>,
}

impl TitleGenres {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let genres = values
            .get(8)
            .ok_or(format!("Failed to extract genre for {title_id}"))
            .map(|&v| v.split(',').map(|v| v.to_string()).collect())?;

        Ok(Self {
            title_id,
            genres,
        })
    }
}

pub async fn parse_title_genres(file_name: &str, table_name: &str, conn: &mut SqliteConnection) -> Result<(), String> {
    println!("-- Inserting Into {table_name} --");
    create_table(table_name, conn).await?;

    let file = File::open(file_name)
        .map_err(|e| format!("Unable to read from {file_name} -> {e}"))?;
    let mut reader = BufReader::new(file);
    let count = (&mut reader).lines().skip(1).count();
    reader.rewind().map_err(|e| format!("Failed to read file {file_name} after counting => {e}"))?;

    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;


    for (i, title_genres) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleGenres::from))
        .enumerate()
    {
        let title_genres = title_genres?;
        for genre in title_genres.genres {
            let query = format!("INSERT INTO {table_name} VALUES($1, $2)");
            sqlx::query(&query)
                .bind(title_genres.title_id)
                .bind(&genre)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {}, into {table_name} => {e}",
                       title_genres.title_id, genre,
                    )
                })?;
        }
        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn create_table(table_name: &str, conn: &mut SqliteConnection) -> Result<(), String> {
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (title_id integer not null, genre text not null, foreign key(title_id) references title(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
