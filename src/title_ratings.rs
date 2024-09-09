use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::utils::{Counter, SqliteParser};

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

impl From<&str> for TitleRatings<BufReader<File>> {
    fn from(file_path: &str) -> Self {
        let file = File::open(&file_path)
            .map_err(|e| format!("Unable to read from {file_path} -> {e}")).unwrap();
        let count = BufReader::new(file).lines().skip(1).count();

        let file =
            File::open(file_path).map_err(|e| format!("Unable to read from {file_path} -> {e}")).unwrap();
        let buf = BufReader::new(file);

        Self{ buf, count }
    }
}

impl Iterator for TitleRatings<BufReader<File>> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl SqliteParser for TitleRatings<BufReader<File>> {
    fn parse(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), String> {
        todo!()
    }
}
