mod name;
mod title;
mod utils;

use std::{env, fs::File};

use name::{get_names, Name};
use sqlx::{Connection, SqliteConnection};
use title::parse_title_basics_tsv;

const NAME_TABLE_NAME: &str = "name";
const NAME_PROFESSION_TABLE_NAME: &str = "name_profession";
const NAME_TITLE_TABLE_NAME: &str = "name_title";

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let path = get_database_path(&args)?;
    let mut conn = SqliteConnection::connect(path)
        .await
        .map_err(|e| format!("Unable to connect to {path} -> {e}"))?;

    parse_title_basics_tsv(&mut conn).await?;


    println!("Finished Converting.");
    Ok(())
}

fn get_database_path(args: &[String]) -> Result<&str, String> {
    let path = args.get(1).unwrap();
    //if Path::new(path).exists() {
    //}

    File::create(path).map_err(|e| format!("Failed to create file {path} => {e}"))?;
    Ok(path)
}

//async fn create_tables(pool: &SqlitePool) -> Result<(), String> {
//    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_TABLE_NAME} (id integer primary key, name text not null, birth_year integer, death_year integer)").as_str())
//        .execute(pool)
//        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;
//
//    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_PROFESSION_TABLE_NAME} (name_id integer not null, profession text not null, foreign key(name_id) references name(id))").as_str())
//        .execute(pool)
//        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;
//
//
//    sqlx::raw_sql(format!("CREATE TABLE IF NOT EXISTS {NAME_TITLE_TABLE_NAME} (name_id integer not null, title_id integer not null, foreign key(name_id) references name(id), foreign key(title_id) references title(id))").as_str())
//        .execute(pool)
//        .await.map_err(|e| format!("Unable to create names table -> {e}"))?;
//
//    Ok(())
//}
//
//async fn fill_name_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
//    let mut tx = pool
//        .begin()
//        .await
//        .map_err(|e| format!("Failed to start transaction => {e}"))?;
//    println!("-- Name Table Progress --");
//    let query = format!("INSERT INTO {NAME_TABLE_NAME} VALUES($1, $2, $3, $4)");
//    for (i, name) in names.iter().enumerate() {
//        sqlx::query(&query)
//            .bind(name.id)
//            .bind(&name.name)
//            .bind(name.birth_date)
//            .bind(name.death_date)
//            .execute(&mut *tx)
//            .await
//            .map_err(|e| {
//                format!(
//                    "Failed to insert {}, {}, {:?}, {:?} into {NAME_TABLE_NAME} => {e}",
//                    name.id, name.name, name.birth_date, name.death_date
//                )
//            })?;
//
//        print_insertion_percentage(i, names.len());
//    }
//    println!();
//
//    tx.commit()
//        .await
//        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
//    println!("Names inserted");
//
//    Ok(())
//}
//
//async fn fill_name_profession_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
//    let mut tx = pool
//        .begin()
//        .await
//        .map_err(|e| format!("Failed to start transaction => {e}"))?;
//
//    println!("-- Name Profession Table Progress --");
//    let query = format!("INSERT INTO {NAME_PROFESSION_TABLE_NAME} VALUES($1, $2)");
//    for (i, name) in names.iter().enumerate() {
//        for profession in name.professions.iter() {
//            sqlx::query(&query)
//                .bind(name.id)
//                .bind(profession)
//                .execute(&mut *tx)
//                .await
//                .map_err(|e| {
//                    format!(
//                        "Failed to insert {}, {} into {NAME_PROFESSION_TABLE_NAME} => {e}",
//                        name.id, profession
//                    )
//                })?;
//        }
//
//        print_insertion_percentage(i, names.len());
//    }
//    println!();
//
//    tx.commit()
//        .await
//        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
//
//    Ok(())
//}
//
//async fn fill_name_title_table(pool: &SqlitePool, names: &[Name]) -> Result<(), String> {
//    let mut tx = pool
//        .begin()
//        .await
//        .map_err(|e| format!("Failed to start transaction => {e}"))?;
//
//    println!("-- Name Title Table Progress --");
//    let query = format!("INSERT INTO {NAME_TITLE_TABLE_NAME} VALUES($1, $2)");
//    for (i, name) in names.iter().enumerate() {
//        for title in name.titles.iter() {
//            sqlx::query(&query)
//                .bind(name.id)
//                .bind(title)
//                .execute(&mut *tx)
//                .await
//                .ok();
//        }
//        print_insertion_percentage(i, names.len());
//    }
//    println!();
//
//    tx.commit()
//        .await
//        .map_err(|e| format!("Failed to commit transactions => {e}"))?;
//    Ok(())
//}
