// Copyright (c) 2026 Neil Pandya

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub act_score: u8,
    pub sat_score: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortPriority {
    FirstName,
    LastName,
    Age,
    ActScore,
    SatScore,
}
