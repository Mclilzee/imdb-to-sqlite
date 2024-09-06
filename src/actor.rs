#[derive(Debug)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub birth_date: Option<u16>,
    pub death_date: Option<u16>,
}

impl Actor {
    pub fn from(line: String) -> Self {
        let values: Vec<&str> = line.split('\t').collect();
        let id: u32 = values.first().unwrap()[2..].parse().unwrap();
        let name = values.get(1).unwrap().to_string();
        let birth_date = values.get(2).and_then(|v| v.parse::<u16>().ok());
        let death_date = values.get(3).and_then(|v| v.parse::<u16>().ok());

        Self {
            id,
            name,
            birth_date,
            death_date,
        }
    }
}

