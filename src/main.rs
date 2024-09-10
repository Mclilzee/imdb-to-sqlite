mod parsers;
mod utils;

use parsers::*;
use sqlx::{Connection, SqliteConnection};
use std::{env, fs::File};

const TITLE_BASICS_FILE: &str = "title.basics.tsv";
const TITLE_TABLE: &str = "title";
const TITLE_GENRES_TABLE: &str = "title_genre";

const TITLE_RATING_FILE: &str = "title.ratings.tsv";
const TITLE_RATING_TABLE: &str = "title_rating";

const TITLE_CREW_FILE: &str = "title.crew.tsv";
const TITLE_DIRECTORS_TABLE: &str = "title_director";

const NAME_BASICS_FILE: &str = "name.basics.tsv";
const NAME_TABLE: &str = "name";
const NAME_PROFESSION_TABLE: &str = "name_profession";
const NAME_TITLE_TABLE: &str = "name_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let path = get_database_path(&args)?;
    let path = "imdb.db";

    let mut conn = SqliteConnection::connect(path)
        .await
        .map_err(|e| format!("Unable to connect to {path} -> {e}"))?;

    if let Err(str) = title::prase_titles(TITLE_BASICS_FILE, TITLE_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = name::parse_names(NAME_BASICS_FILE, NAME_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = name_titles::parse_name_titles(NAME_BASICS_FILE, NAME_TITLE_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = title_genres::parse_title_genres(TITLE_BASICS_FILE, TITLE_GENRES_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = title_ratings::parse_title_ratings(TITLE_RATING_FILE, TITLE_RATING_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = name_professions::parse_name_professions(NAME_BASICS_FILE, NAME_PROFESSION_TABLE, &mut conn).await {
        println!("{str}");
    }

    if let Err(str) = title_directors::parse_title_directors(TITLE_CREW_FILE, TITLE_DIRECTORS_TABLE, &mut conn).await {
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
