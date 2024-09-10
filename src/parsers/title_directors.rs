struct TitleDirectors {
    title_id: u32,
    directors: Vec<u32>,
}

impl TitleDirectors {
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let title_id = values
            .first()
            .and_then(|&s| s[2..].parse::<u32>().ok())
            .ok_or(format!("Failed to parse title_id from {line}"))?;

        let directors = values
            .get(1)
            .map(|&s| {
                s.split(',')
                    .filter_map(|v| v[2..].parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .ok_or(format!("Failed to parse directores from {line}"))?;

        Ok(Self {
            title_id,
            directors,
        })
    }
}

//43   │ tt0000042   nm0617588   \N
//44   │ tt0000043   nm0617588   \N
//45   │ tt0000044   nm0617588   \N
