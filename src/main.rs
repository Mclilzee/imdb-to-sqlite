mod actor;
mod title;

use actor::{get_actors, Actor};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use title::get_titles;

const MAX_CONNECTIONS: u32 = 10;
const DATABASE_NAME: &str = "imdb.db";
const ACTOR_TABLE_NAME: &str = "actor";
const ACTOR_PROFESSION_TABLE_NAME: &str = "actor_profession";
const ACTOR_TITLES_TABLE_NAME: &str = "actor_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_NAME)
        .await
        .map_err(|e| format!("Unable to connect to {DATABASE_NAME} -> {e}"))?;

    create_tables(&pool).await?;
    let actors = get_actors()?;
    let titles = get_titles()?;

    titles.iter().for_each(|t| println!("{t:?}"));
    fill_actor_table(&pool, &actors).await?;
    fill_actor_role_table(&pool, &actors).await?;
    fill_actor_title_table(&pool, &actors).await?;

    println!("Finished Converting.");
    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> Result<(), String> {

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_PROFESSION_TABLE_NAME} (actor_id integer not null, profession text not null, foreign key(actor_id) references actor(id))").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_TITLES_TABLE_NAME} (actor_id integer not null, title integer not null, foreign key(actor_id) references actor(id))").as_str())
        .execute(pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;


    Ok(())
}

async fn fill_actor_table(pool: &SqlitePool, actors: &[Actor]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;
    let query = format!("INSERT INTO {ACTOR_TABLE_NAME} VALUES($1, $2, $3, $4)");
    let mut percentage: u8 = 0;
    for (i, actor) in actors.iter().enumerate() {
        sqlx::query(&query)
            .bind(actor.id)
            .bind(&actor.name)
            .bind(actor.birth_date)
            .bind(actor.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert {actor:?} into {ACTOR_TABLE_NAME} => {e}"))?;

        if i % 100000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_insertion_percentage(percentage, ACTOR_TABLE_NAME);
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    println!("Actors inserted");

    Ok(())
}

async fn fill_actor_role_table(pool: &SqlitePool, actors: &[Actor]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    let query = format!("INSERT INTO {ACTOR_PROFESSION_TABLE_NAME} VALUES($1, $2)");
    let mut percentage: u8 = 0;
    for (i, actor) in actors.iter().enumerate() {
        for profession in actor.professions.iter() {
            sqlx::query(&query)
                .bind(actor.id)
                .bind(profession)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {} into {ACTOR_PROFESSION_TABLE_NAME} => {e}",
                        actor.id, profession
                    )
                })?;
        }

        if i % 100000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_insertion_percentage(percentage, ACTOR_PROFESSION_TABLE_NAME);
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    println!("Actors inserted");

    Ok(())
}

async fn fill_actor_title_table(pool: &SqlitePool, actors: &[Actor]) -> Result<(), String> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;

    let query = format!("INSERT INTO {ACTOR_TITLES_TABLE_NAME} VALUES($1, $2)");
    let mut percentage: u8 = 0;
    for (i, actor) in actors.iter().enumerate() {
        for title in actor.titles.iter() {
            sqlx::query(&query)
                .bind(actor.id)
                .bind(title)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {} into {ACTOR_TITLES_TABLE_NAME} => {e}",
                        actor.id, title
                    )
                })?;
        }

        if i % 100000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_insertion_percentage(percentage, ACTOR_TITLES_TABLE_NAME);
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    Ok(())
}

fn print_insertion_percentage(n: u8, table_name: &str) {
    assert!((0..=100).contains(&n));

    println!("-- Table {table_name} --");
    print!("[");
    for i in 0..=100 {
        if i <= n {
            print!("#");
        } else {
            print!("-");
        }
    }

    println!("] {n}%");
}
