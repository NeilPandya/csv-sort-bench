// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::{SortPriority, Student};

pub fn sort(students: &mut [Student], priority: SortPriority) -> f64 {
    let comparator = get_comparator(priority);
    let start = std::time::Instant::now();
    let n = students.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && comparator(&students[j - 1], &students[j]) == std::cmp::Ordering::Greater {
            students.swap(j - 1, j);
            j -= 1;
        }
    }

    start.elapsed().as_secs_f64() * 1000.0
}
