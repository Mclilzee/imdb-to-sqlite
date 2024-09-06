use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const ACTORS_TSV_FILE: &str = "name.basics.tsv";

const DATABASE_NAME: &str = "imdb.db";
const ACTORS_TABLE_NAME: &str = "actors";

struct Actor<'a> {
    id: u32,
    name: &'a str,
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
        .max_connections(5)
        .connect(DATABASE_NAME)
        .await.map_err(|_| format!("Unable to connect to {DATABASE_NAME}"))?;

    sqlx::query("CREATE TABLE IF NOT EXISTS {$1} ( id integer primary key, name text not null)")
        .bind(ACTORS_TABLE_NAME)
        .fetch_one(&pool)
        .await.map_err(|_| "Unable to create actors table".to_string())?;

    Ok(pool)
}

async fn fill_names_database(pool: &SqlitePool) -> Result<(), String> {
    let names = File::open(ACTORS_TSV_FILE).map_err(|_| format!("Unable to read from {ACTORS_TSV_FILE}"))?;
    let reader = BufReader::new(names);
    let _ = reader
        .lines()
        .skip(1)
        .map_while(Result::ok)
        .map(|l| {
            let values: Vec<&str> = l.split('\t').collect();
            let id: u32 = values.first().unwrap()[2..].parse().unwrap();
            let name = values.get(1).unwrap().to_string();
            sqlx::query("INSERT INTO $1 VALUES($2, '$3')")
                .bind(ACTORS_TABLE_NAME)
                .bind(id)
                .bind(name)
                .fetch_one(pool)
        })
        .collect::<Vec<_>>();


    Ok(())
}
