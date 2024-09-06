use futures::future::join_all;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_CONNECTIONS: u32 = 100;
const ACTORS_TSV_FILE: &str = "name.basics.tsv";
const DATABASE_NAME: &str = "imdb.db";
const ACTORS_TABLE_NAME: &str = "actors";

#[derive(Debug)]
struct Actor {
    id: u32,
    name: String,
    birth_date: Option<u16>,
    death_date: Option<u16>,
}

impl Actor {
    fn from(line: String) -> Self {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap().to_string();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());

        Self {
            id,
            name,
            birth_date,
            death_date,
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), String> {
    let pool = create_tables().await?;
    fill_names_database(&pool).await?;

    println!("Finished Converting.");
    Ok(())
}

async fn create_tables() -> Result<SqlitePool, String> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_NAME)
        .await
        .map_err(|e| format!("Unable to connect to {DATABASE_NAME} -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTORS_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(&pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;

    Ok(pool)
}

async fn fill_names_database(pool: &SqlitePool) -> Result<(), String> {
    println!("Parsing actors.");
    let names = File::open(ACTORS_TSV_FILE)
        .map_err(|e| format!("Unable to read from {ACTORS_TSV_FILE} -> {e}"))?;
    let reader = BufReader::new(names);
    let actors = reader
        .lines()
        .skip(1)
        .map_while(Result::ok)
        .map(Actor::from)
        .collect::<Vec<_>>();

    println!("Parsed actors, Preparing transactions");

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;
    let query = format!("INSERT INTO {ACTORS_TABLE_NAME} VALUES($1, $2, $3, $4)");
    let mut percentage: u8 = 0;
    for (i, actor) in actors.iter().enumerate() {
        sqlx::query(&query)
            .bind(actor.id)
            .bind(&actor.name)
            .bind(actor.birth_date)
            .bind(actor.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert {actor:?} into {ACTORS_TABLE_NAME} => {e}"))?;

        if i % 1000000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_percentage(percentage);
            }
        }
    }

    println!("Started commiting transactions");
    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

fn print_percentage(n: u8) {
    assert!((0..=100).contains(&n));

    print!("[");
    for i in 0..=100 {
        if i <= n {
            print!("#");
        } else {
            print!("-");
        }
    }

    println!("] {n}%");
}
