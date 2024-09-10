mod parsers;
mod utils;

use parsers::*;
use sqlx::{Connection, SqliteConnection};
use std::{env, fs::File, process::exit};

const TITLE_FILE_NAME: &str = "title.basics.tsv";
const TITLE_TABLE_NAME: &str = "title";

const NAME_TABLE_NAME: &str = "name";
const NAME_PROFESSION_TABLE_NAME: &str = "name_profession";
const NAME_TITLE_TABLE_NAME: &str = "name_title";
const TITLE_RATING_TABLE_NAME: &str = "title_rating";
const TITLE_RATING_FILE_NAME: &str = "title.ratings.tsv";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let path = get_database_path(&args)?;
    let mut conn = SqliteConnection::connect(path)
        .await
        .map_err(|e| format!("Unable to connect to {path} -> {e}"))?;

    if let Err(str) = title::prase_titles_file(TITLE_FILE_NAME, TITLE_TABLE_NAME, &mut conn).await {
        println!("{str}");
        println!("Critical Error, unable to insert one of main tables, the program will terminate");
        exit(1);
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
