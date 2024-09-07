#[derive(Debug)]
pub struct Title {
    pub id: u32,
    pub primary_name: String,
    pub original_name: String,
    pub title_type: String,
    pub release_date: Option<u16>,
    pub end_date: Option<u16>,
    pub genres: Vec<String>
}

impl Title {
   //2   │ tt0000001   short   Carmencita  Carmencita  0   1894    \N  1   Documentary,Short
    fn from(line: String) -> Result<Self, String> {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let title_type = 
        let primary_name = values.get(1).map(|&s| s.to_string()).unwrap();
        let original_name = values.get(2).map(|&s| s.to_string()).unwrap();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());
        let professions = values
            .get(4)
            .map(|&v| v.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
            .unwrap();

        let titles = values
            .get(5)
            .map(|v| v.split(','))
            .map(|v| v.flat_map(|n| n[2..].parse::<u32>()).collect::<Vec<_>>())
            .unwrap();

        Ok(Title {
            id,
            primary_name,
            original_name,
            release_date,
            end_date,
            genres,
        })
    }
}

pub fn get_actors() -> Result<Vec<Actor>, String> {
    let names = File::open(ACTORS_TSV_FILE)
        .map_err(|e| format!("Unable to read from {ACTORS_TSV_FILE} -> {e}"))?;

    BufReader::new(names)
        .lines()
        .skip(1)
        .map(|l| l.map_err(|e| format!("Unable to read line -> {e}")))
        .map(|l| l.and_then(Actor::from))
        .collect()
}


