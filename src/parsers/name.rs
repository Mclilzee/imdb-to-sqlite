use std::{fs::File, io::{BufRead, BufReader}};
pub struct Name {
    pub id: u32,
    pub name: String,
    pub birth_date: Option<u16>,
    pub death_date: Option<u16>,
}

impl Name {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap().to_string();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());

        Ok(Self {
            id,
            name,
            birth_date,
            death_date,
        })
    }
}

pub fn get_names() -> Result<Vec<Name>, String> {
    println!("Parsing {NAMES_TSV_FILE}");
    let names = File::open(NAMES_TSV_FILE)
        .map_err(|e| format!("Unable to read from {NAMES_TSV_FILE} -> {e}"))?;

    BufReader::new(names)
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Name::from))
        .collect()
}

async fn create_tables(pool: &SqlitePool) -> Result<(), String> {
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;




    Ok(())
}

async fn fill_name_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;
    println!("-- Name Table Progress --");
    let query = format!("INSERT INTO {NAME_TABLE_NAME} VALUES($1, $2, $3, $4)");
    for (i, name) in names.iter().enumerate() {
        sqlx::query(&query)
            .bind(name.id)
            .bind(&name.name)
            .bind(name.birth_date)
            .bind(name.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {:?}, {:?} into {NAME_TABLE_NAME} => {e}",
                    name.id, name.name, name.birth_date, name.death_date
                )
            })?;

        print_insertion_percentage(i, names.len());
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    println!("Names inserted");

    Ok(())
}


async fn fill_name_title_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    println!("-- Name Title Table Progress --");
    let query = format!("INSERT INTO {NAME_TITLE_TABLE_NAME} VALUES($1, $2)");
    for (i, name) in names.iter().enumerate() {
        for title in name.titles.iter() {
            sqlx::query(&query)
                .bind(name.id)
                .bind(title)
                .execute(&mut *tx)
                .await
                .ok();
        }
        print_insertion_percentage(i, names.len());
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    Ok(())
}
