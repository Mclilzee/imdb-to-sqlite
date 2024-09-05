use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let names = File::open("../name.basics.tsv")?;
    let reader = BufReader::new(names);
    reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|l| l.split('\t').map(str::to_owned).collect::<Vec<_>>()).for_each(|l| println!("{l}"));

    println!("Finished Converting.");
    Ok(())
}
