use std::{
    fs::File,
    io::{self, BufReader},
};

pub mod day_1;
pub mod day_2;


/// Attempt to open the file at `file_path` and return a BufReader<File>.
pub fn open_file(file_path: &str) -> io::Result<BufReader<File>> {
    // Open the file and read contents
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

