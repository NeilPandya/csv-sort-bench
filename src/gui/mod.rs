// Copyright (c) 2026 Neil Pandya

// GUI Module Gateway
// This file will define how we run the GUI application.

pub mod app;

pub fn run_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Sort Benchmark",
        options,
        Box::new(|_cc| Ok(Box::new(app::SortBenchApp::default()))),
    )
}
