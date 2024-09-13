use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "Parse and convert imdb TSV (Tab Saparated Values) Into a Sqlite table")]
pub struct Args {
    /// File name of the database, if file doesn't exist, then file will be created
    pub path: String,

    /// Overwrite option will erase the content of File if it exists and write it from the beggining
    #[arg(short = 'o', long = "overwrite")]
    pub overwrite: bool,
}
