use std::{fs::File, io::BufReader, path::PathBuf};
use sqlx::SqliteConnection;

pub fn percentage_printer(progress: usize, total: usize) {
    if progress % 10000 != 0 {
        return;
    }

    let n = progress as f32 / total as f32 * 100.0 + 2.0;
    let n = n as u8;
    print!("\r[");
    for _ in 0..n {
        print!("#");
    }

    for _ in n..101 {
        print!("-");
    }

    print!("] {:02}%", u8::min(n, 100));
}

pub trait SqliteInserter {
    async fn insert(self, conn: &mut SqliteConnection) -> Result<(), String>;
    async fn create_table(&self, conn: &mut SqliteConnection) -> Result<(), String>;
}


