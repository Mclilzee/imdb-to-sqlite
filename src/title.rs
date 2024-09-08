use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::utils::percentage_printer;

const TITLE_TSV_FILE: &str = "title.basics.tsv";
const TITLE_TABLE_NAME: &str = "title";
const GENRE_TABLE_NAME: &str = "title_genre";

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

struct Genre {
    title_id: u32,
    name: String,
}

impl Genre {
    fn from(line: String) -> Result<Vec<Self>, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let genre = values.get(4).map(|&v| {
            v.split(',')
                .map(|v| v.to_string())
                .map(|genre| Self {
                    title_id,
                    name: genre,
                })
                .collect::<Vec<Genre>>()
        });

        return genre.ok_or(format!("Failed to extract genre for {title_id}"));
    }
}

pub async fn parse_title_basics_tsv(conn: &mut SqliteConnection) -> Result<(), String> {
    println!("Creating {TITLE_TABLE_NAME} Table...");

    create_tables(conn).await?;

    Ok(())
}

async fn create_tables(conn: &mut SqliteConnection) -> Result<(), String> {
    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {TITLE_TABLE_NAME} (id integer primary key, primary_name text not null, original_name text not null, title_type text not null, release_date integer, end_date integer)").as_str())
        .execute(&mut *tx)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {GENRE_TABLE_NAME} (title_id integer not null, genre text not null, foreign key(title_id) references title(id))").as_str())
        .execute(&mut *tx)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn insert_titles(conn: &mut SqliteConnection) -> Result<(), String> {
    println!("-- Creating {TITLE_TABLE_NAME} --");
    let names = File::open(TITLE_TSV_FILE)
        .map_err(|e| format!("Unable to read from {TITLE_TSV_FILE} -> {e}"))?;
    let mut reader = BufReader::new(names);
    let count = (&mut reader).lines().skip(1).count();

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
        let query = format!("INSERT INTO {TITLE_TABLE_NAME} VALUES($1, $2, $3, $4, $5, $6)");
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
                    "Failed to insert {}, {}, {}, {}, {:?}, {:?} into {TITLE_TABLE_NAME} => {e}",
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

async fn insert_genres(conn: &mut SqliteConnection) -> Result<(), String> {
    println!("-- Creating {GENRE_TABLE_NAME} --");
    let names = File::open(TITLE_TSV_FILE)
        .map_err(|e| format!("Unable to read from {TITLE_TSV_FILE} -> {e}"))?;
    let mut reader = BufReader::new(names);
    let count = (&mut reader).lines().skip(1).count();

    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    for (i, genres) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Genre::from))
        .enumerate()
    {
        for genre in genres? {
            let query = format!("INSERT INTO {GENRE_TABLE_NAME} VALUES($1, $2)");
            sqlx::query(&query)
                .bind(genre.title_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {}, into {GENRE_TABLE_NAME} => {e}",
                        genre.title_id, genre.name,
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
