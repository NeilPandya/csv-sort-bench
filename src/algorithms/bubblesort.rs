// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::Record;

pub fn sort(records: &mut [Record], column_index: usize) -> f64 {
    let comparator = get_comparator(column_index);
    let start = std::time::Instant::now();
    let n = records.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if comparator(&records[j], &records[j + 1]) == std::cmp::Ordering::Greater {
                records.swap(j, j + 1);
            }
        }
    }
    start.elapsed().as_secs_f64() * 1000.0
}

// ----------  TESTS  -------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*; // brings `sort` and `Record` into scope

    #[test]
    fn sorts_by_given_column() {
        // Build a tiny deterministic data set
        // Use the vec! macro directly; Record is just an alias for Vec<String>
        let mut data = vec![
            vec!["John".into(), "Doe".into(), "25".into()], // first record
            vec!["Alice".into(), "Smith".into(), "22".into()], // second record
            vec!["Bob".into(), "Brown".into(), "23".into()], // third record
        ];

        // Sort by the third column (the age field, index = 2)
        // The signature is `fn sort(data: &mut [Record], column_index: usize)`
        // Each element of `data` is a `Vec<String>`, i.e., a `Record`.
        sort(&mut data, 2);

        // Verify that the first record now holds the smallest age value (22)
        // `data[0].get(2).unwrap()` returns `&String`; comparing it to a `&str`
        // is allowed because `&String` implements `PartialEq<&str>`.
        assert_eq!(data[0].get(2).unwrap(), "22");
    }

    #[test]
    fn does_nothing_on_empty_slice() {
        // An empty slice should not panic and should return quickly
        let mut empty: Vec<Vec<String>> = Vec::new();
        let duration = sort(&mut empty, 0);
        // The function should finish quickly; we just assert that it returns a finite f64.
        assert!((0.0..=1.0).contains(&duration));
    }
}
