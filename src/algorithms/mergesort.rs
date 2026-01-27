// Copyright (c) 2026 Neil Pandya

use crate::algorithms::get_comparator;
use crate::models::Record;
use std::cmp::Ordering;

pub fn sort(records: &mut [Record], column_index: usize) -> f64 {
    let comparator = get_comparator(column_index);
    let start = std::time::Instant::now();
    merge_sort(records, &comparator);
    start.elapsed().as_secs_f64() * 1000.0
}

fn merge_sort<F>(slice: &mut [Record], compare: &F)
where
    F: Fn(&Record, &Record) -> Ordering,
{
    let mid = slice.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut slice[..mid], compare);
    merge_sort(&mut slice[mid..], compare);

    let mut ret = Vec::with_capacity(slice.len());
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < slice.len() {
        if compare(&slice[i], &slice[j]) != Ordering::Greater {
            ret.push(slice[i].clone());
            i += 1;
        } else {
            ret.push(slice[j].clone());
            j += 1;
        }
    }

    while i < mid {
        ret.push(slice[i].clone());
        i += 1;
    }
    while j < slice.len() {
        ret.push(slice[j].clone());
        j += 1;
    }

    slice.clone_from_slice(&ret);
}
