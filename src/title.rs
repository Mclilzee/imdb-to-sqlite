use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use sqlx::SqlitePool;

const TITLE_TSV_FILE: &str = "title.basics.tsv";
const TITLE_TABLE_NAME: &str = "title";

pub struct TitleWithoutGenre {
    pub id: u32,
    pub primary_name: String,
    pub original_name: String,
    pub title_type: String,
    pub release_date: Option<u16>,
    pub end_date: Option<u16>,
}

impl TitleWithoutGenre {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let title_type = values.get(1).map(|&s| s.to_string()).unwrap();
        let primary_name = values.get(2).map(|&s| s.to_string()).unwrap();
        let original_name = values.get(3).map(|&s| s.to_string()).unwrap();

        let release_date = values.get(5).and_then(|v| v.parse::<u16>().ok());
        let end_date = values.get(6).and_then(|v| v.parse::<u16>().ok());

        Ok(Self {
            id,
            title_type,
            primary_name,
            original_name,
            release_date,
            end_date,
        })
    }
}


pub struct TitleWithGenre {
    pub id: u32,
    pub primary_name: String,
    pub original_name: String,
    pub title_type: String,
    pub release_date: Option<u16>,
    pub end_date: Option<u16>,
    pub genre: Vec<String>,
}

impl TitleWithoutGenre {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let title_type = values.get(1).map(|&s| s.to_string()).unwrap();
        let primary_name = values.get(2).map(|&s| s.to_string()).unwrap();
        let original_name = values.get(3).map(|&s| s.to_string()).unwrap();

        let release_date = values.get(5).and_then(|v| v.parse::<u16>().ok());
        let end_date = values.get(6).and_then(|v| v.parse::<u16>().ok());

        Ok(Self {
            id,
            title_type,
            primary_name,
            original_name,
            release_date,
            end_date,
        })
    }
}

pub async fn parse_title(pool: &SqlitePool) -> Result<(), String> {
    println!("Parsing {TITLE_TSV_FILE}...");
    let names = File::open(TITLE_TSV_FILE)
        .map_err(|e| format!("Unable to read from {TITLE_TSV_FILE} -> {e}"))?;

    let mut timer = std::time::Instant::now();
    BufReader::new(&names).lines().skip(1).count();
    println!("Took to count {:?}", timer.elapsed());

    {
        timer = std::time::Instant::now();
        BufReader::new(&names)
            .lines()
            .skip(1)
            .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
            .map(|l| l.and_then(Title::from))
            .collect::<Vec<_>>();
        println!("Took to map them with genres {:?}", timer.elapsed());
    }

    {
        timer = std::time::Instant::now();
        BufReader::new(&names)
            .lines()
            .skip(1)
            .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
            .map(|l| l.and_then(Title::from))
            .collect::<Vec<_>>();
        println!("Took to map them without genres {:?}", timer.elapsed());
    }

    {
        timer = std::time::Instant::now();
        BufReader::new(&names)
            .lines()
            .skip(1)
            .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
            .map(|l| l.and_then(Title::from))
            .collect::<Vec<_>>();
        println!("Took to map only genres {:?}", timer.elapsed());
    }

    //let mut tx = pool
    //    .begin()
    //    .await
    //    .map_err(|e| format!("Failed to start transaction => {e}"))?;

    //for (i, title) in BufReader::new(names)
    //    .lines()
    //    .skip(1)
    //    .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
    //    .map(|l| l.and_then(Title::from)).enumerate()
    //{
    //    let title = title?;
    //    let query = format!("INSERT INTO {TITLE_TABLE_NAME} VALUES($1, $2, $3, $4, $5, $6)");
    //    sqlx::query(&query)
    //        .bind(title.id)
    //        .bind(&title.primary_name)
    //        .bind(&title.original_name)
    //        .bind(&title.title_type)
    //        .bind(title.release_date)
    //        .bind(title.end_date)
    //        .execute(&mut *tx)
    //        .await
    //        .map_err(|e| {
    //            format!(
    //                "Failed to insert {}, {}, {}, {}, {:?}, {:?} into {TITLE_TABLE_NAME} => {e}",
    //                title.id,
    //                title.primary_name,
    //                title.original_name,
    //                title.title_type,
    //                title.release_date,
    //                title.end_date
    //            )
    //        })?;
    //}

    //tx.commit()
    //    .await
    //    .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

fn print_insertion_percentage(index: usize, size: usize) {
    if index % 100000 != 0 {
        return;
    }

    let n = index as f32 / size as f32 * 100.0 + 2.0;
    let n = n as u8;
    print!("\r[");
    for _ in 0..n {
        print!("#");
    }

    for _ in n..101 {
        print!("-");
    }

    print!("] {:02}%", u8::min(n, 100));
}
