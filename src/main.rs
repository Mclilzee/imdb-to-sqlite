mod name;
mod title;

use name::{get_names, Name};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use title::{get_titles, Title};

const MAX_CONNECTIONS: u32 = 10;
const DATABASE_NAME: &str = "imdb.db";
const NAME_TABLE_NAME: &str = "name";
const NAME_PROFESSION_TABLE_NAME: &str = "name_profession";
const NAME_TITLES_TABLE_NAME: &str = "name_title";
const TITLE_TABLE_NAME: &str = "title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_NAME)
        .await
        .map_err(|e| format!("Unable to connect to {DATABASE_NAME} -> {e}"))?;

    create_tables(&pool).await?;
    {
        println!("Parsing names lines");
        let names = get_names()?;
        fill_name_table(&pool, &names).await?;
        fill_name_profession_table(&pool, &names).await?;
        fill_name_title_table(&pool, &names).await?;
    }

    {
        println!("Parsing title lines");
        let titles = get_titles()?;
        fill_title_basics_table(&pool, &titles).await?;
    }

    println!("Finished Converting.");
    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> Result<(), String> {
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_PROFESSION_TABLE_NAME} (name_id integer not null, profession text not null, foreign key(name_id) references name(id))").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {TITLE_TABLE_NAME} (id integer primary key, primary_name text not null, original_name text not null, title_type text not null, release_date integer, end_date integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;

    Ok(())
}

async fn fill_name_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;
    let query = format!("INSERT INTO {NAME_TABLE_NAME} VALUES($1, $2, $3, $4)");
    println!("-- Name Table Progress --");
    for (i, name) in names.iter().enumerate() {
        sqlx::query(&query)
            .bind(name.id)
            .bind(&name.name)
            .bind(name.birth_date)
            .bind(name.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {:?}, {:?} into {NAME_TABLE_NAME} => {e}",
                    name.id, name.name, name.birth_date, name.death_date
                )
            })?;

        if i % 100000 == 0 {
            print_insertion_percentage(i, names.len());
        }
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    println!("Names inserted");

    Ok(())
}

async fn fill_name_profession_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    println!("-- Name Profession Table Progress --");
    let query = format!("INSERT INTO {NAME_PROFESSION_TABLE_NAME} VALUES($1, $2)");
    for (i, name) in names.iter().enumerate() {
        for profession in name.professions.iter() {
            sqlx::query(&query)
                .bind(name.id)
                .bind(profession)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {} into {NAME_PROFESSION_TABLE_NAME} => {e}",
                        name.id, profession
                    )
                })?;
        }

        if i % 100000 == 0 {
            print_insertion_percentage(i, names.len());
        }
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}

async fn fill_name_title_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    println!("-- Name Title Table Progress --");
    let query = format!("INSERT INTO {NAME_TITLES_TABLE_NAME} VALUES($1, $2)");
    for (i, name) in names.iter().enumerate() {
        for title in name.titles.iter() {
            sqlx::query(&query)
                .bind(name.id)
                .bind(title)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {} into {NAME_TITLES_TABLE_NAME} => {e}",
                        name.id, title
                    )
                })?;
        }

        if i % 100000 == 0 {
            print_insertion_percentage(i, names.len());
        }
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    Ok(())
}

async fn fill_title_basics_table(pool: &SqlitePool, titles: &[Title]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    println!("-- Title Table Progress --");
    let query = format!("INSERT INTO {TITLE_TABLE_NAME} VALUES($1, $2, $3, $4, $5, $6)");
    for (i, title) in titles.iter().enumerate() {
        sqlx::query(&query)
            .bind(title.id)
            .bind(&title.primary_name)
            .bind(&title.original_name)
            .bind(&title.title_type)
            .bind(title.release_date)
            .bind(title.end_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                format!(
                    "Failed to insert {}, {}, {}, {}, {:?}, {:?} into {TITLE_TABLE_NAME} => {e}",
                    title.id,
                    title.primary_name,
                    title.original_name,
                    title.title_type,
                    title.release_date,
                    title.end_date
                )
            })?;

        if i % 100000 == 0 {
            print_insertion_percentage(i, titles.len());
        }
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    Ok(())
}

fn print_insertion_percentage(index: usize, size: usize) {
    let n: u8 = ((index as f32 / size as f32) * 100.0) as u8;
    assert!((0..=100).contains(&n));
    print!("\r[");
    for i in 0..=100 {
        if i <= n {
            print!("#");
        } else {
            print!("-");
        }
    }

    print!("] {n}%");
}
