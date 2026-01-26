// Copyright (c) 2026 Neil Pandya

use crate::algorithms::{self, BenchResult};
use crate::data_gen;
use crate::models::{SortPriority, Student};
use eframe::egui;
use egui::Widget;
use egui_plot::{Bar, BarChart, Plot};

pub struct SortBenchApp {
    students: Vec<Student>,
    results: Vec<BenchResult>,
    priority: SortPriority,
    num_students: usize,
}

impl Default for SortBenchApp {
    fn default() -> Self {
        Self {
            students: Vec::new(),
            results: Vec::new(),
            priority: SortPriority::FirstName,
            num_students: 100,
        }
    }
}

impl eframe::App for SortBenchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sorting Algorithm Benchmarker");

            // ----- Horizontal row: label + slider + button -----
            ui.horizontal(|ui| {
                ui.label("Number of Students");
                egui::Slider::new(&mut self.num_students, 10..=500).ui(ui);

                if ui.button("Start New Session (Generate Data)").clicked() {
                    self.students = data_gen::generate_students(self.num_students);
                    let _ = data_gen::save_to_csv(&self.students, "students.csv");
                    self.results.clear();
                }
            });

            // ----- Combo box for sort priority -----
            egui::ComboBox::from_label("Sort Priority")
                .selected_text(format!("{:?}", self.priority))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.priority, SortPriority::FirstName, "First Name");
                    ui.selectable_value(&mut self.priority, SortPriority::LastName, "Last Name");
                    ui.selectable_value(&mut self.priority, SortPriority::Age, "Age");
                    ui.selectable_value(&mut self.priority, SortPriority::ActScore, "ACT Score");
                    ui.selectable_value(&mut self.priority, SortPriority::SatScore, "SAT Score");
                });

            ui.add_space(10.0);
            if ui.button("Run Benchmarks").clicked() && !self.students.is_empty() {
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
        }); // ← closes the CentralPanel::default().show(...) block
    }
}

impl SortBenchApp {
    fn run_benchmarks(&mut self) {
        self.results.clear();

        // Standard Sort
        let mut data = self.students.clone();
        let time = algorithms::standardsort::sort(&mut data, self.priority);
        self.results.push(BenchResult {
            name: "Std Sort".to_string(),
            duration_ms: time,
        });

        // Merge Sort
        let mut data = self.students.clone();
        let time = algorithms::mergesort::sort(&mut data, self.priority);
        self.results.push(BenchResult {
            name: "Merge Sort".to_string(),
            duration_ms: time,
        });

        // Quick Sort
        let mut data = self.students.clone();
        let time = algorithms::quicksort::sort(&mut data, self.priority);
        self.results.push(BenchResult {
            name: "Quick Sort".to_string(),
            duration_ms: time,
        });

        // Bubble Sort
        let mut data = self.students.clone();
        let time = algorithms::bubblesort::sort(&mut data, self.priority);
        self.results.push(BenchResult {
            name: "Bubble Sort".to_string(),
            duration_ms: time,
        });

        // Insertion Sort
        let mut data = self.students.clone();
        let time = algorithms::insertionsort::sort(&mut data, self.priority);
        self.results.push(BenchResult {
            name: "Insertion Sort".to_string(),
            duration_ms: time,
        });
    }
}
