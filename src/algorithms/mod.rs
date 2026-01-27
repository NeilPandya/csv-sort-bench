// Copyright (c) 2026 Neil Pandya

// Algorithm Module Gateway

pub mod bubblesort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod standardsort;

use crate::models::Record;
use std::cmp::Ordering;

/// Returns a comparator for the given column index.
/// Attempts to compare numerically if both values are valid floats.
pub fn get_comparator(column_index: usize) -> impl Fn(&Record, &Record) -> Ordering {
    move |a, b| {
        let val_a = a.get(column_index).map(|s| s.as_str()).unwrap_or("");
        let val_b = b.get(column_index).map(|s| s.as_str()).unwrap_or("");

        if let (Ok(num_a), Ok(num_b)) = (val_a.parse::<f64>(), val_b.parse::<f64>()) {
            num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
        } else {
            val_a.cmp(val_b)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchResult {
    pub name: String,
    pub duration_ms: f64,
}
