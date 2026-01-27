/// Copyright (c) 2026 Neil Pandya
use crate::models::Record;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn detect_delimiter(path: &Path) -> u8 {
    let candidates = [b',', b';', b'\t', b'|'];
    let mut best_delimiter = b',';

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        if let Some(Ok(first_line)) = reader.lines().next() {
            let mut max_count = 0;
            for &c in &candidates {
                let count = first_line.chars().filter(|&ch| ch == c as char).count();
                if count > max_count {
                    max_count = count;
                    best_delimiter = c;
                }
            }
        }
    }
    best_delimiter
}

pub fn save_csv(path: &Path, headers: &[String], records: &[Record]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    // Write headers
    wtr.write_record(headers)?;

    // Write data rows
    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}
