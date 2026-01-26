// Copyright (c) 2026 Neil Pandya

// Data Generation Module
// This file will define how we generate student data.

use crate::models::Student;
use fake::Fake;
use fake::faker::name::en::{FirstName, LastName};
use rand::Rng;
use std::error::Error;

pub fn generate_students(count: usize) -> Vec<Student> {
    let mut rng = rand::thread_rng();
    let mut students = Vec::with_capacity(count);

    for _ in 0..count {
        students.push(Student {
            // This generates actual strings like "John" or "Smith"
            first_name: FirstName().fake(),
            last_name: LastName().fake(),
            age: rng.gen_range(17..25),
            act_score: rng.gen_range(1..37),
            sat_score: rng.gen_range(400..1601),
        });
    }
    students
}

pub fn save_to_csv(students: &[Student], path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    for student in students {
        wtr.serialize(student)?;
    }
    wtr.flush()?;
    Ok(())
}
