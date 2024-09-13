mod config;
mod parsers;
mod utils;

use clap::Parser;
use config::Args;
use parsers::*;
use sqlx::{Connection, SqliteConnection};
use std::{fs::File, path::Path};

const TITLE_BASICS_FILE: &str = "title.basics.tsv";
const TITLE_TABLE: &str = "title";
const TITLE_GENRES_TABLE: &str = "title_genre";

const TITLE_RATING_FILE: &str = "title.ratings.tsv";
const TITLE_RATING_TABLE: &str = "title_rating";

const TITLE_CREW_FILE: &str = "title.crew.tsv";
const TITLE_DIRECTORS_TABLE: &str = "title_director";
const TITLE_WRITERS_TABLE: &str = "title_writer";

const TITLE_EPISODE_FILE: &str = "title.episode.tsv";
const TITLE_EPISODE_TABLE: &str = "title_episode";

const TITLE_PRINCIPALS_FILE: &str = "title.principals.tsv";
const TITLE_JOB_TABLE: &str = "title_job";
const TITLE_CHARACTERS_TABLE: &str = "title_character";

const NAME_BASICS_FILE: &str = "name.basics.tsv";
const NAME_TABLE: &str = "name";
const NAME_PROFESSION_TABLE: &str = "name_profession";
const NAME_TITLE_TABLE: &str = "name_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    if !Path::new(&args.path).exists() {
        File::create(&args.path)
            .map_err(|e| format!("Failed to create file {} => {e}", args.path))?;
    }

    let mut conn = SqliteConnection::connect(&args.path)
        .await
        .map_err(|e| format!("Unable to connect to {} -> {e}", args.path))?;

    if args.full || args.lite || args.core || args.name {
        if let Err(str) = names::parse_names(NAME_BASICS_FILE, NAME_TABLE, &mut conn, &args).await {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.name_profession {
        if let Err(str) = name_professions::parse_name_professions(
            NAME_BASICS_FILE,
            NAME_PROFESSION_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.lite || args.core || args.title {
        if let Err(str) =
            titles::prase_titles(TITLE_BASICS_FILE, TITLE_TABLE, &mut conn, &args).await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.lite || args.name_title {
        if let Err(str) =
            name_titles::parse_name_titles(NAME_BASICS_FILE, NAME_TITLE_TABLE, &mut conn, &args)
                .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.title_genre {
        if let Err(str) = title_genres::parse_title_genres(
            TITLE_BASICS_FILE,
            TITLE_GENRES_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.title_rating {
        if let Err(str) = title_ratings::parse_title_ratings(
            TITLE_RATING_FILE,
            TITLE_RATING_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.title_director {
        if let Err(str) = title_directors::parse_title_directors(
            TITLE_CREW_FILE,
            TITLE_DIRECTORS_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.title_writer {
        if let Err(str) = title_writers::parse_title_writers(
            TITLE_CREW_FILE,
            TITLE_WRITERS_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.full || args.title_episode {
        if let Err(str) = title_episodes::parse_title_episodes(
            TITLE_EPISODE_FILE,
            TITLE_EPISODE_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.extra || args.title_job {
        if let Err(str) =
            title_jobs::parse_title_jobs(TITLE_PRINCIPALS_FILE, TITLE_JOB_TABLE, &mut conn, &args)
                .await
        {
            eprintln!("\n{str}");
        }
    }

    if args.extra || args.title_character {
        if let Err(str) = title_characters::parse_title_characters(
            TITLE_PRINCIPALS_FILE,
            TITLE_CHARACTERS_TABLE,
            &mut conn,
            &args,
        )
        .await
        {
            eprintln!("\n{str}");
        }
    }

    println!("Finished Converting.");
    Ok(())
}
