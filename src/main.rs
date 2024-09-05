use std::{fs::File, io::{self, BufReader, Error}};

fn main() -> Result<(), Error> {
    let names = File::open("./dictionary.json")?;
    let reader = BufReader::new(dictionary_file);

    println!("Finished Converting.");
    Ok(())
}
