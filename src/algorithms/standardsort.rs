// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::{SortPriority, Student};

pub fn sort(students: &mut [Student], priority: SortPriority) -> f64 {
    let comparator = get_comparator(priority);
    let start = std::time::Instant::now();
    students.sort_by(comparator);
    start.elapsed().as_secs_f64() * 1000.0
}
