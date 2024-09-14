# Introduction

A tool to convert IMDb dataset to sqlite for ease of use. This tool is not affialated with IMDb, this tool was built to be used as educational or personal purposes and does not support using the dataset without following [IMDb Non-Commersial license](https://developer.imdb.com/non-commercial-datasets/)

## Installation

Download one of the release pre-built binaries,
or use Rust's Cargo to build the sourcecode

## About

Parse and convert IMDb TSV (Tab Saparated Values) Into a Sqlite tables

This program require IMDb dataset in their original names, format and unzipped.
You can find it by going to
[IMDb dataset](https://developer.imdb.com/non-commercial-datasets/) free of charge.
To use the dataset you need to comply with their Non-Commercial liecense,
otherwise this program is not complicit in any lisence breaking.

The dataset information can be found at IMDb official site. The tables are separated into 3 categories, core, joining, extra. The core tables are the two main ones (title, name) which requires no foreign keys. Joining tables which will have foreign key to one or both of the core tables. The extra is also a joining tables but they are really slow to parse and contains over 80 mill rows each.

The options below can be toggled at the same time to mix and match to your liking. You can choose per category, stand alone titles, or full, lite and extra versions which is pre-defined tables the rows which have their foreign keys constrait not found will be skipped from being inserted with no error shown. Otherwise errors will be showend and the insertion will stop, example: Trying to insert another row with the same primary key and previous one.

Make sure to choose the overwrite option if you want to insert the same tables again otherwise you will be having duplicate entries for the joining tables with no primary keys.


## Usage
Usage: imdbsql [OPTIONS] <PATH>

Arguments:
  <PATH>
          File name of the database, if file doesn't exist, then file will be created

Options:
      --log
          Log option to show insertion errors, examples: Insertion errors due to forgein key constraint, or primary key already exists. By default those errors are skiped and not printed out

  -o, --overwrite
          Overwrite option will the tables before being inserted into. Will only erase the tables that are toggle through the table flags

  -l, --lite
          Lite option will toggle the core tables and the one joining table between them (title, name, name_title)

  -c, --core
          Core option will toggle the two core tables, (title, name)
          
          Files required (title.basics.tsv, name.basics.tsv)

  -a, --all
          All option will toggle all the tables except the slowest ones (not really all but almost all) which are (title_job, title_character)
          
          Files required (title.basics.tsv, title.ratings.tsv, title.crew.tsv, title.episode.tsv, name.basics.tsv)

  -e, --extra
          Extra option will toggle the extra tables which are the slowest and will take a long time to parse, (title_job, title_character)
          
          files required (title.principals.tsv)

      --name
          Name option will toggle the name table parsing.
          
          File required (name.bascis.tsv)
          
          schema: (id PRIMARY KEY, name TEXT NOT NULL, birth_year INTEGER, death_year INTEGER)

      --name_profession
          Name_Profession option will toggle the name_profession table parsing.
          
          File required (name.basics.tsv)
          
          schema: (name_id INTEGER NOT NULL, profession TEXT NOT NULL, FOREIGN KEY(name_id) REFERENCES name(id))

      --title
          Title option will toggle the title table parsing.
          
          File required (title.basics.tsv)
          
          schema: (id INTEGER PRIMARY KEY, primary_name TEXT NOT NULL, original_name TEXT NOT NULL, title_type TEXT NOT NULL, release_date INTEGER, end_date INTEGER)

      --name_title
          Name_Title option will toggle the name_title table parsing.
          
          File required (name.basics.tsv)
          
          schema: (name_id INTEGER NOT NULL, title_id INTEGER NOT NULL, FOREIGN KEY(name_id) REFERENCES name(id), FOREIGN KEY(title_id) REFERENCES title(id))

      --title_genre
          Title_Genre option will toggle the title_genre table parsing.
          
          File required (title.basics.tsv)
          
          schema: (title_id INTEGER NOT NULL, genre TEXT NOT NULL, FOREIGN KEY(title_id) REFERENCES title(id))

      --title_rating
          Title_Rating option will toggle the name_title table parsing.
          
          File required (title.ratings.tsv)
          
          schema: (title_id INTEGER NOT NULL, average_rating REAL NOT NULL, votes INTEGER NOT NULL, FOREIGN KEY(title_id) REFERENCES title(id))

      --title_director
          Title_Director option will toggle the title_director table parsing.
          
          File required (title.crew.tsv)
          
          schema: (title_id INTEGER NOT NULL, name_id INTEGER NOT NULL, FOREIGN KEY(title_id) REFERENCES title(id), FOREIGN KEY(name_id) REFERENCES name(id))

      --title_writer
          Title_Writer option will toggle the title_writer table parsing.
          
          File required (title.crew.tsv)
          
          schema: (title_id NOT NULL, name_id INTEGER NOT NULL, FOREIGN KEY(title_id) REFERENCES title(id), FOREIGN KEY(name_id) REFERENCES name(id))

      --title_episode
          Title_Episode option will toggle the title_episode table parsing.
          
          File required (title.episode.tsv)
          
          schema: (title_episode_id INTEGER NOT NULL, title_series_id INTEGER NOT NULL, episode_number INTEGER, season_number INTEGER, FOREIGN KEY(title_episode_id) REFERNECES title(id), FOREIGN KEY(title_series_id) REFERENCES title(id))

      --title_job
          Title_Job option will toggle the title_job table parsing.
          
          File required (title.principals.tsv)
          
          schema: (title_id INTEGER NOT NULL, name_id INTEGER NOT NULL, category TEXT NOT NULL, job TEXT, FOREIGN KEY(title_id) REFERENCES title(id), FOREIGN KEY(name_id) REFERENCES name(id))

      --title_character
          Title_Character option will toggle the title_character table parsing.
          
          File required (title.principals.tsv)
          
          schema: (title_id INTEGER NOT NULL, name_id INTEGER NOT NULL, character TEXT TEXT NOT NULL, FOREIGN KEY(title_id) REFERENCESE title(id), FOREIGN KEY(name_id) REFERENCES name(id))

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
