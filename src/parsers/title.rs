use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};
use crate::utils::percentage_printer;

pub struct Title {
    id: u32,
    primary_name: String,
    original_name: String,
    title_type: String,
    release_date: Option<u16>,
    end_date: Option<u16>,
}

impl Title {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let title_type = values.get(1).map(|&s| s.to_string()).unwrap();
        let primary_name = values.get(2).map(|&s| s.to_string()).unwrap();
        let original_name = values.get(3).map(|&s| s.to_string()).unwrap();

        let release_date = values.get(5).and_then(|v| v.parse::<u16>().ok());
        let end_date = values.get(6).and_then(|v| v.parse::<u16>().ok());

        Ok(Self {
            id,
            title_type,
            primary_name,
            original_name,
            release_date,
            end_date,
        })
    }
}

pub async fn prase_titles(file_name: &str, table_name: &str, conn: &mut SqliteConnection) -> Result<(), String> {
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

    for (i, title) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Title::from))
        .enumerate()
    {
        let title = title?;
        let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3, $4, $5, $6)");
        sqlx::query(&query)
            .bind(title.id)
            .bind(&title.primary_name)
            .bind(&title.original_name)
            .bind(&title.title_type)
            .bind(title.release_date)
            .bind(title.end_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {}, {}, {:?}, {:?} into {table_name} => {e}",
                    title.id,
                    title.primary_name,
                    title.original_name,
                    title.title_type,
                    title.release_date,
                    title.end_date
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
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (id integer primary key, primary_name text not null, original_name text not null, title_type text not null, release_date integer, end_date integer)").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
