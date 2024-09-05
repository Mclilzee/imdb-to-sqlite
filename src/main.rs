use sqlite::Connection;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), String> {
    let conn = create_databases()?;
    let names =
        File::open("name.basics.tsv").map_err(|_| String::from("Failed to read names database"))?;
    let reader = BufReader::new(names);
    reader.lines().map_while(Result::ok).for_each(|l| {
        for (i, v) in l.split('\t').enumerate() {}
    });

    println!("Finished Converting.");
    Ok(())
}

fn create_databases() -> Result<Connection, String> {
    let conn = Connection::open("names.db")
        .map_err(|_| String::from("Failed to open database names.db"))?;
    conn.execute("create table if not exists actors ( id integer primary key, name text not null)")
        .map_err(|_| String::from("Could not create table"))?;

    Ok(conn)
}
