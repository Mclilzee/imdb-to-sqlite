mod actor;

use actor::{get_actors, Actor};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const MAX_CONNECTIONS: u32 = 10;
const DATABASE_NAME: &str = "imdb.db";
const ACTOR_TABLE_NAME: &str = "actor";
const ACTOR_ROLE_TABLE_NAME: &str = "actor_role";
const ACTOR_TITLES_NAME: &str = "actor_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool = create_tables().await?;
    let actors = get_actors()?;
    fill_actor_table(&pool, &actors).await?;
    fill_actor_role_table(&pool, &actors).await?;
    fill_actor_title_table(&pool, &actors).await?;

    println!("Finished Converting.");
    Ok(())
}

async fn create_tables() -> Result<SqlitePool, String> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_NAME)
        .await
        .map_err(|e| format!("Unable to connect to {DATABASE_NAME} -> {e}"))?;

    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(&pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_ROLE_TABLE_NAME} (actor_id foreign key, role text not null)").as_str())
        .execute(&pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;
    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {ACTOR_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
        .execute(&pool)
        .await.map_err(|e| format!("Unable to create actors table -> {e}"))?;

    Ok(pool)
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
                print_percentage(percentage, "Actor Insertions");
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
            .map_err(|e| format!("Failed to insert {actor:?} into {ACTORS_TABLE_NAME} => {e}"))?;

        if i % 100000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_percentage(percentage);
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
    println!("Parsed {} actors, Preparing Actors transactions", actors.len());
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;
    let query = format!("INSERT INTO {ACTORS_TABLE_NAME} VALUES($1, $2, $3, $4)");
    let mut percentage: u8 = 0;
    for (i, actor) in actors.iter().enumerate() {
        sqlx::query(&query)
            .bind(actor.id)
            .bind(&actor.name)
            .bind(actor.birth_date)
            .bind(actor.death_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert {actor:?} into {ACTORS_TABLE_NAME} => {e}"))?;

        if i % 100000 == 0 {
            let new_percentage: f32 = (i as f32 / actors.len() as f32) * 100.0;
            let new_percentage = new_percentage as u8;
            if new_percentage > percentage {
                percentage = new_percentage;
                print_percentage(percentage);
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
    println!("Actors inserted");

    Ok(())
}

fn print_percentage(n: u8, title: &str) {
    assert!((0..=100).contains(&n));

    println!("-- {title} --");
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
