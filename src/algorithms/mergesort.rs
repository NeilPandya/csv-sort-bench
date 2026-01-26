use crate::algorithms::get_comparator;
use crate::models::{SortPriority, Student};
use std::cmp::Ordering;

pub fn sort(students: &mut [Student], priority: SortPriority) -> f64 {
    let comparator = get_comparator(priority);
    let start = std::time::Instant::now();
    merge_sort(students, &comparator);
    start.elapsed().as_secs_f64() * 1000.0
}

fn merge_sort<F>(slice: &mut [Student], compare: &F)
where
    F: Fn(&Student, &Student) -> Ordering,
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
