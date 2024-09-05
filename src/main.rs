use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let names = File::open("name.basics.tsv")?;
    let reader = BufReader::new(names);
    reader.lines().map_while(Result::ok).for_each(|l| {
        for (i, v) in l.split('\t').enumerate() {

        };
    });

    println!("Finished Converting.");
    Ok(())
}
