use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"Parse and convert imdb TSV (Tab Saparated Values) Into a Sqlite tables.
The tables can be found at https://developer.imdb.com/non-commercial-datasets/ make sure to read the License before using. The tables are required in their original format and name, unzipped.

All the table information is there but in our .schema it is adjusted differently separated into 3 categories, core, joining, extra. The core tables are the two main ones (title, name) which requires no foreign keys. joining tables will have foreign key to one of the core ones. The extra is also a joining tables but they are really slow to parse and contains over 40 mill rows

The options bellow can be toggled together to choose which tables to parse. You can choose per category, stand alone titles, or full, lite and extra versions which is pre-defined tables the rows which have their foreign keys constrait not found will be skipped from being inserted with no error shown. Otherwise errors will be showend and the insertion will stop, example: Trying to insert another row with the same primary key and previous one.

Make sure to choose the overwrite option if you want to insert the same tables again otherwise you will be having duplicate entries for the joining tables with no primary keys."#
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

    /// This will parse the title table
    #[arg(short = 't', long = "title")]
    pub title: bool,

}
