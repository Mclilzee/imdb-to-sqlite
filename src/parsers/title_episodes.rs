use crate::{config::Args, utils::percentage_printer};
use sqlx::{Connection, SqliteConnection};
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

struct TitleEpisode {
    title_episode_id: u32,
    title_series_id: u32,
    season_number: Option<u32>,
    episode_number: Option<u32>,
}

impl TitleEpisode {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_episode_id: u32 = values
            .first()
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse title_episode_id from {line}"))?;

        let title_series_id: u32 = values
            .get(1)
            .and_then(|s| s.get(2..))
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Failed to parse title_series_id from {line}"))?;

        let season_number = values.get(2).and_then(|s| s.parse().ok());
        let episode_number = values.get(3).and_then(|s| s.parse().ok());

        Ok(Self {
            title_episode_id,
            title_series_id,
            season_number,
            episode_number,
        })
    }
}

pub async fn parse_title_episodes(
    file_name: &str,
    table_name: &str,
    conn: &mut SqliteConnection,
    args: &Args,
) -> Result<(), String> {

    create_table(table_name, conn, args.overwrite).await?;
    let file =
        File::open(file_name).map_err(|e| format!("Unable to read from {file_name} -> {e}"))?;
    let mut reader = BufReader::new(file);
    let count = (&mut reader).lines().skip(1).count();
    println!("-- Inserting {count} entries into {table_name} --");

    reader
        .rewind()
        .map_err(|e| format!("Failed to read file {file_name} after counting => {e}"))?;

    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    for (i, title_episode) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleEpisode::from))
        .enumerate()
    {
        let title_episode = title_episode?;
        let query = format!("INSERT INTO {table_name} VALUES($1, $2, $3, $4)");
        let _ = sqlx::query(&query)
            .bind(title_episode.title_episode_id)
            .bind(title_episode.title_series_id)
            .bind(title_episode.season_number)
            .bind(title_episode.episode_number)
            .execute(&mut *tx)
            .await
            .inspect_err(|e| {
                if args.log {
                    eprintln!(
                        "Failed to insert {}, {}, {:?}, {:?}, into {table_name} => {e}",
                        title_episode.title_episode_id,
                        title_episode.title_series_id,
                        title_episode.episode_number,
                        title_episode.season_number
                    );
                }
            });

        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn create_table(
    table_name: &str,
    conn: &mut SqliteConnection,
    overwrite: bool,
) -> Result<(), String> {
    if overwrite {
        sqlx::raw_sql(format!("DROP TABLE {table_name}").as_str())
            .execute(&mut *conn)
            .await
            .map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;
    }
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {table_name} (title_episode_id integer not null, title_series_id integer not null, episode_number integer, season_number integer, foreign key(title_episode_id) references title(id), foreign key(title_series_id) references title(id))").as_str())
        .execute(conn)
        .await.map_err(|e| format!("Unable to create {table_name} table -> {e}"))?;

    Ok(())
}
