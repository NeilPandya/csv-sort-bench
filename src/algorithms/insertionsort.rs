// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::Record;

pub fn sort(records: &mut [Record], column_index: usize) -> f64 {
    let comparator = get_comparator(column_index);
    let start = std::time::Instant::now();
    let n = records.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && comparator(&records[j - 1], &records[j]) == std::cmp::Ordering::Greater {
            records.swap(j - 1, j);
            j -= 1;
        }
    }

    start.elapsed().as_secs_f64() * 1000.0
}

// ----------  TESTS  -------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*; // brings `sort` and `Record` into scope

    #[test]
    fn sorts_small_vec_correctly() {
        // Build a tiny deterministic data set.
        // Because `Record` is just an alias for `Vec<String>`,
        // we construct each record directly with `vec!`.
        let mut data = vec![
            vec!["c".into(), "1".into()], // key = "1"
            vec!["a".into(), "3".into()], // key = "3"
            vec!["b".into(), "2".into()], // key = "2"
        ];

        // Sort by the second column (index = 1) – the numeric value.
        sort(&mut data, 1);

        // After sorting, the keys must be in ascending order.
        // `data.iter().map(|r| r.get(1))` yields `Option<&String>`.
        // Use `filter_map` to unwrap only the `Some` values.
        let sorted_keys: Vec<&String> = data.iter().filter_map(|r| r.get(1)).collect();

        assert_eq!(sorted_keys, vec!["1", "2", "3"]); // ascending order
    }

    #[test]
    fn leaves_empty_slice_unchanged() {
        // An empty slice should not panic and should return a finite duration.
        let mut empty: Vec<Vec<String>> = Vec::new();
        let duration = sort(&mut empty, 0);
        assert!((0.0..=1.0).contains(&duration));
    }
}
