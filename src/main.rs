mod name;
mod title_basics;
mod title_ratings;
mod utils;

use sqlx::{Connection, SqliteConnection};
use std::{env, fs::File};
use title_ratings::TitleRatingsInserter;
use utils::SqliteInserter;

const NAME_TABLE_NAME: &str = "name";
const NAME_PROFESSION_TABLE_NAME: &str = "name_profession";
const NAME_TITLE_TABLE_NAME: &str = "name_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let path = get_database_path(&args)?;
    let mut conn = SqliteConnection::connect(path)
        .await
        .map_err(|e| format!("Unable to connect to {path} -> {e}"))?;

    title_basics::parse_titles(&mut conn).await?;
    let file = File::open("title.ratings.tsv")
        .map_err(|e| format!("Failed to read title.ratings.tsv => {e}"))?;
    TitleRatingsInserter::new(file, "title_ratings".to_string())?.insert(&mut conn).await?;

    //title_basics::parse_genres(&mut conn).await?;

    println!("Finished Converting.");
    Ok(())
}

fn get_database_path(args: &[String]) -> Result<&str, String> {
    let path = args.get(1).unwrap();
    //if Path::new(path).exists() {
    //}

    File::create(path).map_err(|e| format!("Failed to create file {path} => {e}"))?;
    Ok(path)
}
