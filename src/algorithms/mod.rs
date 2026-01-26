// Copyright (c) 2026 Neil Pandya

// Algorithm Module Gateway
// This file will define how we compare students and export the individual algorithm modules.

pub mod bubblesort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod standardsort;

use crate::models::{SortPriority, Student};
use std::cmp::Ordering;

pub fn get_comparator(priority: SortPriority) -> impl Fn(&Student, &Student) -> Ordering {
    move |a, b| match priority {
        SortPriority::FirstName => a.first_name.cmp(&b.first_name),
        SortPriority::LastName => a.last_name.cmp(&b.last_name),
        SortPriority::Age => a.age.cmp(&b.age),
        SortPriority::ActScore => a.act_score.cmp(&b.act_score),
        SortPriority::SatScore => a.sat_score.cmp(&b.sat_score),
    }
}

#[derive(Debug, Clone)]
pub struct BenchResult {
    pub name: String,
    pub duration_ms: f64,
}
