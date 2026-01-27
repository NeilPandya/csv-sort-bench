// Copyright (c) 2026 Neil Pandya

/// A generic record representing a row in a CSV.
/// This alias allows the codebase to remain agnostic of the underlying
/// storage container, making future optimizations (like using Box or SmallVec) easier.
pub type Record = Vec<String>;

/// Represents the results of a benchmark run for a specific algorithm.
pub use crate::algorithms::BenchResult;
