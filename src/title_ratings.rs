use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use crate::utils::{percentage_printer, two_decimal, SqliteInserter};
use sqlx::{Connection, SqliteConnection};

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
            .map(two_decimal)
            .unwrap_or_default();

        let votes = values.get(2).unwrap().parse::<u32>().unwrap_or_default();

        Ok(Self {
            title_id,
            average_rating,
            votes,
        })
    }
}

pub struct TitleRatingsInserter {
    reader: BufReader<File>,
    table_name: String,
    count: usize,
}

impl TitleRatingsInserter {
    pub fn new(file: File, table_name: String) -> Result<Self, String> {
        let mut reader = BufReader::new(file);
        let count = (&mut reader).lines().skip(1).count();
        reader
            .rewind()
            .map_err(|e| format!("Failed to rewind reader => {e}"))?;

        Ok(Self {
            reader,
            count,
            table_name,
        })
    }
}

impl SqliteInserter for TitleRatingsInserter {
    async fn insert(self, conn: &mut SqliteConnection) -> Result<(), String> {
        self.create_table(conn).await?;

        let mut tx = conn
            .begin()
            .await
            .map_err(|e| format!("Failed to start transaction => {e}"))?;

        println!("-- Inserting Into {} Table --", self.table_name);
        for (i, title_rating) in self
            .reader
            .lines()
            .skip(1)
            .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
            .map(|l| l.and_then(TitleRating::from))
            .enumerate()
        {
            let query = format!("INSERT INTO {} VALUES($1, $2, $3)", self.table_name);
            let title_rating = title_rating?;
            sqlx::query(&query)
                .bind(title_rating.title_id)
                .bind(title_rating.average_rating)
                .bind(title_rating.votes)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {}, {} into {} => {e}",
                        title_rating.title_id,
                        title_rating.average_rating,
                        title_rating.votes,
                        self.table_name,
                    )
                })?;
            percentage_printer(i, self.count);
        }
        println!();

        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transactions => {e}"))?;

        Ok(())
    }

    async fn create_table(&self, conn: &mut SqliteConnection) -> Result<(), String> {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (title_id integer not null, average_rating real not null, votes integer not null, foreign key(title_id) references title(id))",
            self.table_name
        );

        sqlx::raw_sql(&query)
            .execute(conn)
            .await
            .map_err(|e| format!("Unable to create {} table -> {e}", self.table_name))?;

        Ok(())
    }
}
