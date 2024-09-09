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
const TITLE_RATING_TABLE_NAME: &str = "title_rating";
const TITLE_RATING_FILE_NAME: &str = "title.rating.tsv";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let path = get_database_path(&args)?;
    let mut conn = SqliteConnection::connect(path)
        .await
        .map_err(|e| format!("Unable to connect to {path} -> {e}"))?;

    parse_title_ratings(&mut conn).await.map_err(|e| println!("{e}"));
    title_basics::parse_titles(&mut conn).await?;

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

async fn parse_title_ratings(conn: &mut SqliteConnection) -> Result<(), String> {
    let file = File::open(TITLE_RATING_FILE_NAME)
        .map_err(|e| format!("Failed to read {TITLE_RATING_FILE_NAME} => {e}"))?;

    TitleRatingsInserter::new(file, TITLE_RATING_TABLE_NAME.to_string())?.insert(conn).await?;

    Ok(())
}
