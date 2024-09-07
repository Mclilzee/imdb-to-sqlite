use std::{fs::File, io::{BufRead, BufReader}};

const ACTORS_TSV_FILE: &str = "name.basics.tsv";

#[derive(Debug)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub birth_date: Option<u16>,
    pub death_date: Option<u16>,
    pub professions: Vec<String>,
    pub titles: Vec<u32>,
}

impl Actor {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap().to_string();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());
        let professions = values
            .get(4)
            .map(|&v| v.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
            .unwrap();

        let titles = values
            .get(5)
            .map(|v| v.split(','))
            .map(|v| v.flat_map(|n| n[2..].parse::<u32>()).collect::<Vec<_>>())
            .unwrap();

        Ok(Self {
            id,
            name,
            birth_date,
            death_date,
            professions,
            titles,
        })
    }
}

pub fn get_actors() -> Result<Vec<Actor>, String> {
    let names = File::open(ACTORS_TSV_FILE)
        .map_err(|e| format!("Unable to read from {ACTORS_TSV_FILE} -> {e}"))?;

    BufReader::new(names)
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Actor::from))
        .collect()
}

