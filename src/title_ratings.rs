use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::{Path, PathBuf},
};

use sqlx::{Connection, SqliteConnection};

use crate::utils::SqliteInserter;

const TITLE_RATINGS_TABLE: &str = "title_rating";

struct TitleRating {
    title_id: u32,
    average_rating: f32,
    votes: u32,
}

impl TitleRating {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id = values.first().unwrap()[2..]
            .parse::<u32>()
            .map_err(|e| format!("File line ''{line}'' contains wrong format => {e}"))?;
        let average_rating = values
            .get(1)
            .unwrap()
            .parse::<f32>()
            .map_err(|e| format!("File line ''{line}'' contains wrong format => {e}"))?;
        let votes = values
            .get(2)
            .unwrap()
            .parse::<u32>()
            .map_err(|e| format!("File line ''{line}'' contains wrong format => {e}"))?;

        Ok(Self {
            title_id,
            average_rating,
            votes,
        })
    }
}

struct TitleRatingsInserter<BufReader> {
    buf: BufReader,
    count: usize,
}

impl From<BufReader<File>> for TitleRatingsInserter<BufReader<File>> {
    fn from(mut buf: BufReader<File>) -> Self {
        let count = (&mut buf).lines().skip(1).count();
        buf.rewind();
        Self { buf, count }
    }
}

impl SqliteInserter for TitleRatingsInserter<BufReader<File>> {
    async fn insert(self, conn: &mut SqliteConnection) -> Result<(), String> {
        Self::create_table(&mut conn);

        let mut tx = conn
            .begin()
            .await
            .map_err(|e| format!("Failed to start transaction => {e}"))?;

        for (i, title_rating) in self
            .buf
            .lines()
            .skip(1)
            .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
            .map(|l| l.and_then(TitleRating::from))
            .enumerate()
        {
            let query = format!("INSERT INTO {GENRE_TABLE_NAME} VALUES($1, $2)");
            let title_rating = title_rating?;
            sqlx::query(&query)
                .bind(title_rating.title_id)
                .bind(title_rating.votes)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {}, {} into {GENRE_TABLE_NAME} => {e}",
                        title_rating.title_id, title_rating.average_rating, title_rating.votes
                    )
                })?;
            percentage_printer(i, count);
        }
        println!();

        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transactions => {e}"))?;

        Ok(())
    }

    async fn create_table(conn: &mut SqliteConnection) {}
}
