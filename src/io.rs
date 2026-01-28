/// Copyright (c) 2026 Neil Pandya
use crate::models::Record;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_csv(
    path: &std::path::Path,
) -> Result<(Vec<String>, Vec<Record>), Box<dyn std::error::Error>> {
    let delimiter = detect_delimiter(path);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(path)?;

    // Extract headers
    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();

    // Extract records
    let records = rdr
        .records()
        .map(|result| result.map(|record| record.iter().map(|s| s.to_string()).collect()))
        .collect::<Result<Vec<Record>, _>>()?;

    Ok((headers, records))
}

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

// ----------  TESTS  -------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn load_csv_reads_basic_comma_delimited() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("basic.csv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "name,age,salary").unwrap();
        writeln!(file, "Alice,30,50000").unwrap();
        writeln!(file, "Bob,25,45000").unwrap();

        let (headers, records) = load_csv(&file_path).unwrap();

        assert_eq!(headers, vec!["name", "age", "salary"]);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["Alice", "30", "50000"]);
        assert_eq!(records[1], vec!["Bob", "25", "45000"]);
    }

    #[test]
    fn load_csv_reads_semicolon_delimited() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("semicolon.csv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "name;age;salary").unwrap();
        writeln!(file, "Alice;30;50000").unwrap();
        writeln!(file, "Bob;25;45000").unwrap();

        let (headers, records) = load_csv(&file_path).unwrap();

        assert_eq!(headers, vec!["name", "age", "salary"]);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["Alice", "30", "50000"]);
        assert_eq!(records[1], vec!["Bob", "25", "45000"]);
    }

    #[test]
    fn load_csv_reads_tab_delimited() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("tab_delimited.tsv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "name\tage\tsalary").unwrap();
        writeln!(file, "Alice\t30\t50000").unwrap();
        writeln!(file, "Bob\t25\t45000").unwrap();

        let (headers, records) = load_csv(&file_path).unwrap();

        assert_eq!(headers, vec!["name", "age", "salary"]);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["Alice", "30", "50000"]);
        assert_eq!(records[1], vec!["Bob", "25", "45000"]);
    }

    #[test]
    fn load_csv_handles_empty_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("empty.csv");
        fs::File::create(&file_path).unwrap(); // Create empty file

        // An empty CSV file should not panic; it either succeeds with empty data
        // or returns a meaningful error. Either way, it should handle gracefully.
        let _result = load_csv(&file_path);
        // Test passes if we reach here without panicking
    }

    #[test]
    fn load_csv_handles_file_with_only_headers() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("headers_only.csv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "name,age,salary").unwrap();

        let (headers, records) = load_csv(&file_path).unwrap();

        assert_eq!(headers, vec!["name", "age", "salary"]);
        assert!(records.is_empty()); // No data rows
    }

    #[test]
    fn detect_delimiter_identifies_comma() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("sample.csv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "header1,header2,header3").unwrap();
        writeln!(file, "val1,val2,val3").unwrap();

        let detected = detect_delimiter(&file_path);
        assert_eq!(detected, b',');
    }

    #[test]
    fn detect_delimiter_identifies_semicolon() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("sample_semicolon.csv");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "header1;header2;header3").unwrap();
        writeln!(file, "val1;val2;val3").unwrap();

        let detected = detect_delimiter(&file_path);
        assert_eq!(detected, b';');
    }

    #[test]
    fn save_csv_writes_well_formed_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("output.csv");

        let headers = vec!["col1".to_string(), "col2".to_string()];
        let records = vec![
            vec!["a".to_string(), "1".to_string()],
            vec!["b".to_string(), "2".to_string()],
        ];

        let result = save_csv(&file_path, &headers, &records);
        assert!(result.is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "col1,col2");
        assert_eq!(lines[1], "a,1");
        assert_eq!(lines[2], "b,2");
    }
}
