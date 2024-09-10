use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use sqlx::{Connection, SqliteConnection};
use crate::utils::percentage_printer;

struct NameTitles {
    name_id: u32,
    titles: Vec<u32>
}

impl NameTitles {

    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let name_id: u32 = values.first().unwrap()[2..].parse().unwrap();

        let titles = values
            .get(5)
            .map(|v| v.split(','))
            .map(|v| v.flat_map(|n| n[2..].parse::<u32>()).collect::<Vec<_>>())
            .unwrap();

        Ok(Self {
            name_id,
            titles
        })

    }
}

pub async fn parse_name_titles(
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

    let query = format!("INSERT INTO {table_name} VALUES($1, $2)");
    for (i, name_title) in reader.lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(NameTitles::from))
        .enumerate() {
        if let Ok(name_title) = name_title {;
            for title in name_title.titles.iter() {
                let result = sqlx::query(&query)
                    .bind(name_title.name_id)
                    .bind(title)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        format!(
                            "Failed to insert {}, {} into {table_name} => {e}",
                            name_title.name_id, title
                        )
                    });
                if let Err(str) = result {
                    println!("{str}");
                }
            }
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
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (name_id integer not null, title_id integer not null, foreign key(name_id) references name(id), foreign key(title_id) references title(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
