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
