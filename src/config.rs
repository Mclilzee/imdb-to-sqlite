use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Parse and convert imdb TSV (Tab Saparated Values) Into a Sqlite table"
)]
pub struct Args {
    /// File name of the database, if file doesn't exist, then file will be created
    pub path: String,

    /// Overwrite option will erase all the content in the file. Only choose it if you want to do a fresh conversion otherwise all the old ones will be replaced.
    #[arg(short = 'O', long = "overwrite")]
    pub overwrite: bool,

    /// Lite option will toggle the core tables and the one joining table between them (title, name, name_title)
    #[arg(short = 'l', long = "lite")]
    pub lite: bool,

    /// Core option will toggle the two core tables, (title, name)
    #[arg(short = 'c', long = "core")]
    pub core: bool,

    /// Full option will toggle all the tables except the slowest ones, (title_principal, title_character, title_locale)
    #[arg(short = 'f', long = "full")]
    pub full: bool,

    /// Extra option will toggle the extra tables which are the slowest and will take a long time to parse, (title_principal, title_character, title_locale)
    #[arg(short = 'e', long = "extra")]
    pub extra: bool,
}
