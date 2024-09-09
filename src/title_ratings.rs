use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::{Path, PathBuf},
};

use crate::utils::{PercentagePrinter, SqliteInserter};

const TITLE_RATINGS_TABLE: &str = "title_rating";

//struct TitleRatings {
//    title_id: u32,
//    average_rating: f32,
//    votes: u32,
//}
//
//impl TitleRatings{
//    fn from(line: String) -> Result<Self, ()> {
//        let values: Vec<&str> = line.split('\t').collect();
//        let title_id = values.first().unwrap()[2..].parse::<u32>().unwrap();
//        let average_rating = values.get(1).unwrap().parse::<f32>().unwrap();
//        let votes = values.get(2).unwrap().parse::<u32>().unwrap();
//
//        Ok(Self {title_id, average_rating, votes})
//    }
//
//}

struct TitleRatings<BufReader> {
    buf: BufReader,
    count: usize,
}

impl From<BufReader<File>> for TitleRatings<BufReader<File>> {
    fn from(mut buf: BufReader<File>) -> Self {
        let count = (&mut buf).lines().skip(1).count();
        buf.rewind();
        Self{ buf, count }
    }
}

impl Iterator for TitleRatings<BufReader<File>> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl SqliteInserter for TitleRatings<BufReader<File>> {
    fn insert(self, conn: &mut sqlx::SqliteConnection) -> Result<(), String> {
        todo!()
    }
}
