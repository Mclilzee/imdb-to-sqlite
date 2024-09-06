use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_CONNECTIONS: u32 = 20;
const ACTORS_TSV_FILE: &str = "name.basics.tsv";
const DATABASE_NAME: &str = "imdb.db";
const ACTORS_TABLE_NAME: &str = "actors";

#[derive(Debug)]
struct Actor {
    id: u32,
    name: String,
    birth_date: Option<u16>,
    death_date: Option<u16>
}

impl Actor {
    fn from(line: String) -> Self {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap().to_string();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());

        Self { id, name, birth_date, death_date }
    }
}

#[tokio::main]
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
    let names = File::open(ACTORS_TSV_FILE)
        .map_err(|e| format!("Unable to read from {ACTORS_TSV_FILE} -> {e}"))?;
    let reader = BufReader::new(names);
    for actor in reader
        .lines()
        .skip(1)
        .map_while(Result::ok)
        .map(Actor::from)
    {
        sqlx::query(format!("INSERT INTO {ACTORS_TABLE_NAME} VALUES($1, $2, $3, $4)").as_str())
            .bind(actor.id)
            .bind(&actor.name)
            .bind(actor.birth_date)
            .bind(actor.death_date)
            .execute(pool)
            .await.map_err(|e| format!("Unable to insert {actor:?} into table {ACTORS_TABLE_NAME} => {e}"))?;
    }

    Ok(())
}
