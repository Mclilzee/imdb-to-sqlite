use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const ACTORS_TSV_FILE: &str = "name.basics.tsv";

const DATABASE_NAME: &str = "imdb.db";
const ACTORS_TABLE_NAME: &str = "actors";

struct Actor<'a> {
    id: u32,
    name: &'a str,
}

fn main() -> Result<(), String> {
    let conn = create_databases()?;
    fill_names_database(&conn)?;

    println!("Finished Converting.");
    Ok(())
}

fn create_databases() -> Result<Connection, String> {
    let conn = Connection::open(DATABASE_NAME)
        .map_err(|_| format!("Failed to open database {DATABASE_NAME}"))?;
    conn.execute(format!("CREATE TABLE IF NOT EXISTS {ACTORS_TABLE_NAME} ( id integer primary key, name text not null)"))
        .map_err(|_| String::from("Could not create table for actor names"))?;

    Ok(conn)
}

fn fill_names_database(conn: &Connection) -> Result<(), String> {
    let names =
        File::open(ACTORS_TSV_FILE).map_err(|_| String::from("Failed to read name.basics.tsv"))?;
    let reader = BufReader::new(names);
    reader.lines().skip(1).map_while(Result::ok).map(|l| {
        let values: Vec<&str> = l.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap();
        conn.execute(format!(
            "INSERT INTO {ACTORS_TABLE_NAME} VALUES({id}, \"{name}\")"
        ))
        .unwrap();
    });

    Ok(())
}
