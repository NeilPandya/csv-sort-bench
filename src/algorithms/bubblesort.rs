use crate::algorithms::get_comparator;
use crate::models::{SortPriority, Student};

pub fn sort(students: &mut [Student], priority: SortPriority) -> f64 {
    let comparator = get_comparator(priority);
    let start = std::time::Instant::now();
    let n = students.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if comparator(&students[j], &students[j + 1]) == std::cmp::Ordering::Greater {
                students.swap(j, j + 1);
            }
        }
    }
    start.elapsed().as_secs_f64() * 1000.0
}
