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
