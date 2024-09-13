use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"Parse and convert imdb TSV (Tab Saparated Values) Into a Sqlite tables.
The tables are available to download from https://developer.imdb.com/non-commercial-datasets/ make sure to read the License before using. The tables are required in their original format and name, unzipped.

You can read the table information from the site, but in our .schema it is adjusted differently, some tables into multiple sqlite tables. The tables are separated into 3 categories, core, joining, extra. The core tables are the two main ones (title, name) which requires no foreign keys. Joining tables which will have foreign key to one or both of the core tables. The extra is also a joining tables but they are really slow to parse and contains over 40 mill rows each.

The options bellow can be toggled at the same time to mix and match to your liking. You can choose per category, stand alone titles, or full, lite and extra versions which is pre-defined tables the rows which have their foreign keys constrait not found will be skipped from being inserted with no error shown. Otherwise errors will be showend and the insertion will stop, example: Trying to insert another row with the same primary key and previous one.

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

    /// Extra option will toggle the extra tables which are the slowest and will take a long time to parse, (title_job, title_character, title_locale)
    #[arg(short = 'e', long = "extra")]
    pub extra: bool,

    /// Name option will toggle the name table parsing. schema: (id PRIMARY KEY, name TEXT NOT NULL, birth_year INTEGER, death_year INTEGER)
    #[arg(long = "name")]
    pub name: bool,

    /// Name_Profession option will toggle the name_profession table parsing. schema: (name_id INTEGER NOT NULL, profession TEXT NOT NULL, FOREIGN KEY(name_id) REFERENCES name(id))
    #[arg(long = "name_profession")]
    pub name_profession: bool,

    /// Title option will toggle the title table parsing. schema: (id INTEGER PRIMARY KEY, primary_name TEXT NOT NULL, original_name TEXT NOT NULL, title_type TEXT NOT NULL, release_date INTEGER, end_date INTEGER)
    #[arg(long = "title")]
    pub title: bool,

    /// Name_Title option will toggle the name_title table parsing. schema: (name_id INTEGER NOT NULL, title_id INTEGER NOT NULL, FOREIGN KEY(name_id) REFERENCES name(id), FOREIGN KEY(title_id) REFERENCES title(id))
    #[arg(long = "name_title")]
    pub name_title: bool,

    /// Title_Genre option will toggle the title_genre table parsing
    #[arg(long = "title_genre")]
    pub title_genre: bool,

    /// Title_Rating option will toggle the name_title table parsing
    #[arg(long = "title_rating")]
    pub title_rating: bool,

    /// Title_Director option will toggle the title_director table parsing
    #[arg(long = "title_director")]
    pub title_director: bool,

    /// Title_Writer option will toggle the title_writer table parsing
    #[arg(long = "title_writer")]
    pub title_writer: bool,

    /// Title_Episode option will toggle the title_episode table parsing
    #[arg(long = "title_episode")]
    pub title_episode: bool,

    /// Title_Job option will toggle the title_job table parsing
    #[arg(long = "title_job")]
    pub title_job: bool,

    /// Title_Character option will toggle the title_character table parsing
    #[arg(long = "title_character")]
    pub title_character: bool,
}
