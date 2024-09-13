use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "Parse and convert imdb TSV (Tab Saparated Values) Into a Sqlite table")]
pub struct Args {
    /// File name of the database, if file doesn't exist, then file will be created
    pub path: String,

    /// Overwrite option will erase all the content in the file. Only choose it if you want to do a fresh conversion otherwise all the old ones will be replaced.
    #[arg(short = 'O', long = "overwrite")]
    pub overwrite: bool,

    /// Lite option will create the core tables and the one joining table between them (title, name, name_title)
    #[arg(short = 'l', long = "lite")]
    pub lite: bool,

    /// Core option will make sure to convert the 2 core tables, (title, name)
    #[arg(short = 'c', long = "core")]
    pub core: bool,
}
