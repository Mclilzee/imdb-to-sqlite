use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use crate::utils::percentage_printer;
use sqlx::{Connection, SqliteConnection};

pub struct Name {
    pub id: u32,
    pub name: String,
    pub birth_date: Option<u16>,
    pub death_date: Option<u16>,
}

impl Name {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values
            .first()
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse id from {line}"))?;

        let name = values
            .get(1)
            .map(|s| s.to_string())
            .ok_or(format!("Failed to parse name from {line}"))?;

        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());

        Ok(Self {
            id,
            name,
            birth_date,
            death_date,
        })
    }
}

pub async fn parse_names(
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

    let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3, $4)");
    for (i, name) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Name::from))
        .enumerate()
    {
        let name = name?;
        sqlx::query(&query)
            .bind(name.id)
            .bind(&name.name)
            .bind(name.birth_date)
            .bind(name.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {:?}, {:?} into {table_name} => {e}",
                    name.id, name.name, name.birth_date, name.death_date
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
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
