// Copyright (c) 2026 Neil Pandya

/// This alias allows the codebase to remain agnostic of the underlying
/// storage container, making future optimizations (like using Box or SmallVec) easier.
pub type Record = Vec<String>;

/// Represents the results of a benchmark run for a specific algorithm.
pub use crate::algorithms::BenchResult;

/// Define error types.
#[derive(Debug, Clone)]
pub enum CsvError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CsvError::FileNotFound(path) => write!(f, "File not found: {}", path),
            CsvError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CsvError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CsvError {}
