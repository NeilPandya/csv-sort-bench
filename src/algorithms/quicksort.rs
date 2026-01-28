// Copyright 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::Record;
use std::cmp::Ordering;

pub fn sort(records: &mut [Record], column_index: usize) -> f64 {
    let comparator = get_comparator(column_index);
    let start = std::time::Instant::now();
    quick_sort(records, &comparator);
    start.elapsed().as_secs_f64() * 1000.0
}

fn quick_sort<F>(slice: &mut [Record], compare: &F)
where
    F: Fn(&Record, &Record) -> Ordering,
{
    if slice.len() <= 1 {
        return;
    }
    let pivot_index = partition(slice, compare);
    quick_sort(&mut slice[0..pivot_index], compare);
    quick_sort(&mut slice[pivot_index + 1..], compare);
}

fn partition<F>(slice: &mut [Record], compare: &F) -> usize
where
    F: Fn(&Record, &Record) -> Ordering,
{
    let pivot_index = slice.len() / 2;
    slice.swap(pivot_index, slice.len() - 1);
    let mut i = 0;
    for j in 0..slice.len() - 1 {
        if compare(&slice[j], &slice[slice.len() - 1]) == Ordering::Less {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, slice.len() - 1);
    i
}

// ----------  TESTS  -------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*; // brings `sort`, `Record`, and any helper functions into scope

    #[test]
    fn sorts_records_by_numeric_column() {
        // Build a small test set with known order
        let mut data = vec![
            vec!["Delta".into(), "40".into()],
            vec!["Alpha".into(), "10".into()],
            vec!["Charlie".into(), "30".into()],
            vec!["Beta".into(), "20".into()],
        ];

        // Sort by the second column (index 1), which contains numeric strings
        sort(&mut data, 1);

        // Collect the sorted keys (second column values)
        let sorted_values: Vec<&String> = data.iter().filter_map(|r| r.get(1)).collect();

        // Assert that the values are in ascending numeric order
        assert_eq!(sorted_values, vec!["10", "20", "30", "40"]);
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
