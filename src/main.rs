mod parsers;
mod utils;

use parsers::*;
use sqlx::{Connection, SqliteConnection};
use std::{env, fs::File, process::exit};

const TITLE_BASICS_FILE: &str = "title.basics.tsv";
const TITLE_TABLE_NAME: &str = "title";
const TITLE_GENRES_TABLE_NAME: &str = "title.basics.tsv";

const TITLE_RATING_FILE_NAME: &str = "title.ratings.tsv";
const TITLE_RATING_TABLE_NAME: &str = "title_rating";

const NAME_BASICS_FILE: &str = "name.basics.tsv";
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

    insert_main_tables(&mut conn).await?;

    if let Err(str) = title_genres::parse_title_genres(TITLE_BASICS_FILE, TITLE_GENRES_TABLE_NAME, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = title_ratings::parse_title_ratings(TITLE_RATING_FILE_NAME, TITLE_RATING_TABLE_NAME, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = name_professions::parse_name_professions(NAME_BASICS_FILE, NAME_PROFESSION_TABLE_NAME, &mut conn).await {
        println!("{str}");
    }


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

async fn insert_main_tables(conn: &mut SqliteConnection) -> Result<(), String> {
    if let Err(str) = title::prase_titles(TITLE_BASICS_FILE, TITLE_TABLE_NAME, conn).await {
        println!("{str}");
        return Err("Critical Error, unable to insert one of main tables, the program will terminate".to_string());
    }

    if let Err(str) = name::parse_names(NAME_BASICS_FILE, NAME_TABLE_NAME, conn).await {
        println!("{str}");
        return Err("Critical Error, unable to insert one of main tables, the program will terminate".to_string());
    }

    if let Err(str) = name_titles::parse_name_titles(NAME_BASICS_FILE, NAME_TITLE_TABLE_NAME, conn).await {
        println!("{str}");
        return Err("Critical Error, unable to insert one of main tables, the program will terminate".to_string());
    }

    Ok(())
}
