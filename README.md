# CSV Sort Bench

A lightweight, modular benchmarking tool for comparing sorting algorithms on arbitrary CSV data.  

## Features

- **Dynamic CSV parsing** – automatically detects delimiters (`,`, `;`, `\t`, `|`, etc.).
- **Multiple sorting algorithms** – Standard Sort, Merge Sort, Quick Sort, Bubble Sort, Insertion Sort.
- **Smart sorting** – attempts numeric parsing for logical ordering, falls back to string comparison.
- **Export sorted data** – Generates a new CSV named `<original>_sorted_by_<columnHeader>.csv`.
- **TUI Front‑end** – Built with `eframe` + `egui` for a clean, interactive interface.
- **Modular architecture** – Core logic separated into `algorithms`, `io`, and `models` modules.

## Getting Started

### Prerequisites

- **Rust** (latest stable)
- **cargo** (included with Rust)

### Build & Run

```bash
# Clone the repo (if not already cloned)
git clone https://github.com/NeilPandya/csv-sort-bench.git
cd csv-sort-bench

# Build in release mode
cargo build --release

# Run the application
./target/release/csv-sort-bench
```

### Usage

1. Click **“Upload CSV File”** and select any CSV file.  
2. The app reads the headers and populates the **“Sort by Column”** dropdown.  
3. Choose a column and click **“Run Benchmarks”** to see execution times for each algorithm.  
4. Click **“Export Sorted CSV”** to save the currently sorted data with a filename like  
   `students_sorted_by_age.csv`.

## Supported CSV Formats

- **Comma‑separated** (`.csv`) – default
- **Semicolon‑separated** (`.csv` or `.tsv` with `;`)
- **Tab‑separated** (`.tsv`)
- **Pipe‑separated** (`.csv` with `|`)

The delimiter is auto‑detected from the first line of the file.

## Development

- **Adding a new algorithm** – Implement the sorting function in `src/algorithms/` and ensure it follows the signature:

  ```rust
  fn sort(data: &mut [Record], column_index: usize) -> f64
  ```

- **Running tests**  

  ```bash
  cargo test
  ```

- **Formatting**  

  ```bash
  cargo fmt
  ```

## License

This project is licensed under the **Apache License, Version 2.0**  
**– see the `LICENSE` file for details.**

---

*Created & Maintained by Neil Pandya*
