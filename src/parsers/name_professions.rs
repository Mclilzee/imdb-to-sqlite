use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use crate::{config::Args, utils::percentage_printer};
use sqlx::{Connection, SqliteConnection};

struct NameProfessions {
    name_id: u32,
    professions: Vec<String>,
}

impl NameProfessions {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let name_id: u32 = values
            .first()
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse name_id from {line}"))?;

        let professions = values
            .get(4)
            .map(|&v| v.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
            .ok_or(format!("Failed to parse professions from {line}"))?;

        Ok(Self {
            name_id,
            professions,
        })
    }
}

pub async fn parse_name_professions(
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

    let query = format!("INSERT INTO {table_name} VALUES($1, $2)");

    for (i, name_profession) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(NameProfessions::from))
        .enumerate()
    {
        let name_profession = name_profession?;
        for profession in name_profession.professions.iter() {
            let _ = sqlx::query(&query)
                .bind(name_profession.name_id)
                .bind(profession)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    if args.log {
                        eprintln!(
                            "Failed to insert {}, {} into {table_name} => {e}",
                            name_profession.name_id, profession
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

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (name_id integer not null, profession text not null, foreign key(name_id) references name(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
