// Copyright (c) 2026 Neil Pandya

use crate::algorithms;
use crate::io;
use crate::models::{Record, BenchResult};
use eframe::egui;
use egui_plot::{Bar, BarChart, Plot};
use std::path::PathBuf;

pub struct SortBenchApp {
    records: Vec<Record>,
    headers: Vec<String>,
    results: Vec<BenchResult>,
    selected_column_index: usize,
    loaded_file_path: Option<PathBuf>,
}

impl Default for SortBenchApp {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            headers: Vec::new(),
            results: Vec::new(),
            selected_column_index: 0,
            loaded_file_path: None,
        }
    }
}

impl eframe::App for SortBenchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CSV Sorting Benchmarker");

            ui.horizontal(|ui| {
                if ui.button("Upload CSV File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv"])
                        .pick_file()
                    {
                        self.load_csv(path);
                    }
                }

                if !self.records.is_empty() && ui.button("Export Sorted CSV").clicked() {
                    self.export_csv();
                }
            });

            if !self.headers.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Sort by Column:");
                    egui::ComboBox::from_id_salt("col_select")
                        .selected_text(&self.headers[self.selected_column_index])
                        .show_ui(ui, |ui| {
                            for (i, header) in self.headers.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_column_index, i, header);
                            }
                        });
                });
            }

            ui.add_space(10.0);
            if ui.button("Run Benchmarks").clicked() && !self.records.is_empty() {
                self.run_benchmarks();
            }

            if !self.results.is_empty() {
                ui.add_space(20.0);
                ui.label("Results (ms):");
                let bars: Vec<Bar> = self
                    .results
                    .iter()
                    .enumerate()
                    .map(|(i, res)| Bar::new(i as f64, res.duration_ms).name(&res.name))
                    .collect();

                Plot::new("Benchmark Results")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(BarChart::new(bars).width(0.5));
                    });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

impl SortBenchApp {
    fn load_csv(&mut self, path: PathBuf) {
        let delimiter = io::detect_delimiter(&path);
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_path(&path)
            .expect("Failed to open CSV file");

        if let Ok(headers) = rdr.headers() {
            self.headers = headers.iter().map(|s| s.to_string()).collect();
            self.records = rdr
                .records()
                .filter_map(|r| r.ok())
                .map(|r| r.iter().map(|s| s.to_string()).collect())
                .collect();
            self.results.clear();
            self.selected_column_index = 0;
            self.loaded_file_path = Some(path);
        }
    }

    fn export_csv(&mut self) {
        if let Some(ref original_path) = self.loaded_file_path {
            // 1. Ensure the data is actually sorted according to current selection
            algorithms::standardsort::sort(&mut self.records, self.selected_column_index);

            let file_stem = original_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let column_name = &self.headers[self.selected_column_index];
            let suggested_name = format!("{}_sorted_by_{}.csv", file_stem, column_name);

            if let Some(save_path) = rfd::FileDialog::new()
                .set_file_name(&suggested_name)
                .add_filter("CSV", &["csv", "tsv", "txt"]) // Expanded filter for ISO standards
                .save_file()
            {
                if let Err(e) = io::save_csv(&save_path, &self.headers, &self.records) {
                    eprintln!("Failed to save CSV: {}", e);
                }
            }
        }
    }

    fn run_benchmarks(&mut self) {
        self.results.clear();
        let algorithms = [
            (
                "Std Sort",
                algorithms::standardsort::sort as fn(&mut [Record], usize) -> f64,
            ),
            ("Merge Sort", algorithms::mergesort::sort),
            ("Quick Sort", algorithms::quicksort::sort),
            ("Bubble Sort", algorithms::bubblesort::sort),
            ("Insertion Sort", algorithms::insertionsort::sort),
        ];

        for (name, sort_fn) in algorithms {
            let mut data = self.records.clone();
            let time = sort_fn(&mut data, self.selected_column_index);
            self.results.push(BenchResult {
                name: name.to_string(),
                duration_ms: time,
            });
        }
    }
}
