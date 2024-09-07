use std::{fs::File, io::{BufRead, BufReader}};

const TITLE_TSV_FILE: &str = "title.basics.tsv";

#[derive(Debug)]
pub struct Title {
    pub id: u32,
    pub primary_name: String,
    pub original_name: String,
    pub title_type: String,
    pub release_date: Option<u16>,
    pub end_date: Option<u16>,
    pub genres: Vec<String>
}

impl Title {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let title_type = values.get(1).map(|&s| s.to_string()).unwrap();
        let primary_name = values.get(2).map(|&s| s.to_string()).unwrap();
        let original_name = values.get(3).map(|&s| s.to_string()).unwrap();

        let release_date = values.get(5).and_then(|v| v.parse::<u16>().ok());
        let end_date = values.get(6).and_then(|v| v.parse::<u16>().ok());
        let genres = values
            .get(4)
            .map(|&v| v.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
            .unwrap();

        Ok(Self {
            id,
            title_type,
            primary_name,
            original_name,
            release_date,
            end_date,
            genres,
        })
    }
}

pub fn get_titles() -> Result<Vec<Title>, String> {
    let names = File::open(TITLE_TSV_FILE)
        .map_err(|e| format!("Unable to read from {TITLE_TSV_FILE} -> {e}"))?;

    BufReader::new(names)
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Title::from))
        .collect()
}


