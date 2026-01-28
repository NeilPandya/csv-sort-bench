// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::Record;

pub fn sort(records: &mut [Record], column_index: usize) -> f64 {
    let comparator = get_comparator(column_index);
    let start = std::time::Instant::now();
    records.sort_by(comparator);
    start.elapsed().as_secs_f64() * 1000.0
}

// ----------  TESTS  -------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*; // brings `sort`, `Record`, and any helper functions into scope

    #[test]
    fn sorts_records_by_string_column() {
        // Build a small test set with known order
        let mut data = vec![
            vec!["zebra".into(), "9".into()],
            vec!["apple".into(), "2".into()],
            vec!["banana".into(), "5".into()],
        ];

        // Sort by the first column (index 0), which contains string keys
        sort(&mut data, 0);

        // Collect the sorted keys (first column values)
        let sorted_keys: Vec<&String> = data.iter().filter_map(|r| r.get(0)).collect();

        // Assert that the keys are in lexicographic order
        assert_eq!(sorted_keys, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn sorts_records_by_numeric_column() {
        // Another test case with numeric-like strings
        let mut data = vec![
            vec!["ItemA".into(), "100".into()],
            vec!["ItemB".into(), "20".into()],
            vec!["ItemC".into(), "3".into()],
        ];

        // Sort by the second column (index 1), which contains numeric strings
        sort(&mut data, 1);

        let sorted_values: Vec<&String> = data.iter().filter_map(|r| r.get(1)).collect();

        // Our smart comparator should order these numerically
        assert_eq!(sorted_values, vec!["3", "20", "100"]);
    }

    #[test]
    fn handles_single_record() {
        let mut data = vec![vec!["Only".into(), "999".into()]];

        sort(&mut data, 0);

        assert_eq!(data[0][0], "Only");
        assert_eq!(data[0][1], "999");
    }

    #[test]
    fn leaves_empty_slice_unchanged() {
        let mut empty: Vec<Vec<String>> = Vec::new();
        let duration = sort(&mut empty, 0);
        assert!((0.0..=1.0).contains(&duration));
    }
}
