struct TitleGenres {
    title_id: u32,
    genres: Vec<String>,
}

impl TitleGenres {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let genres = values
            .get(8)
            .ok_or(format!("Failed to extract genre for {title_id}"))
            .map(|&v| v.split(',').map(|v| v.to_string()).collect())?;

        Ok(Self {
            title_id,
            genres,
        })
    }
}

pub async fn parse_genres(conn: &mut SqliteConnection) -> Result<(), String> {
    println!("-- Inserting Into {GENRE_TABLE_NAME} --");
    let file = File::open(TITLE_TSV_FILE)
        .map_err(|e| format!("Unable to read from {TITLE_TSV_FILE} -> {e}"))?;
    let mut reader = BufReader::new(file);

    let count = (&mut reader).lines().skip(1).count();
    reader.rewind();

    let mut tx = conn
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction => {e}"))?;


    for (i, title_genres) in reader
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(TitleGenres::from))
        .enumerate()
    {
        let title_genres = title_genres?;
        for genre in title_genres.genres {
            let query = format!("INSERT INTO {GENRE_TABLE_NAME} VALUES($1, $2)");
            sqlx::query(&query)
                .bind(title_genres.title_id)
                .bind(&genre)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to insert {}, {}, into {GENRE_TABLE_NAME} => {e}",
                       title_genres.title_id, genre,
                    )
                })?;
        }
        percentage_printer(i, count);
    }
    println!();

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transactions => {e}"))?;

    Ok(())
}
