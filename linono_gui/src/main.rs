#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use linono_extractor;
use chrono::NaiveDate;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Linono",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    releases: Option<linono_extractor::Releases>,
}

fn show_release_date(ui: &mut egui::Ui, date: Option<NaiveDate>) {
    if let Some(date) = date {
        ui.label(date.format("%Y-%m-%d").to_string());
    } else {
        ui.label("TBA");
    }
}

fn show_saga_releases(ui: &mut egui::Ui, saga: &str, rels: &[linono_extractor::Release]) {
    ui.heading(saga);
    egui::Grid::new(format!("table_{}", saga)).show(ui, |ui| {
        ui.label("Title");
        ui.label("Release Date");
        ui.end_row();
        for rel in rels {
            ui.label(&rel.title);
            show_release_date(ui, rel.release_date);
            ui.end_row();
        }
    });
    ui.separator();
}

fn show_coming_releases(ui: &mut egui::Ui, coming: &[linono_extractor::Release]) {
    ui.heading("Coming Releases");
    egui::Grid::new("coming").show(ui, |ui| {
        ui.label("Saga");
        ui.label("Title");
        ui.label("Release Date");
        ui.end_row();
        for rel in coming {
            ui.label(&rel.saga);
            ui.label(&rel.title);
            show_release_date(ui, rel.release_date);
            ui.end_row();
        }
    });
}

impl Default for MyApp {
    fn default() -> Self {
        let releases = linono_extractor::Releases::load().ok();
        Self { releases }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(releases) = &self.releases {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (saga, rels) in &releases.all {
                        show_saga_releases(ui, saga, rels);
                    }
                    show_coming_releases(ui, &releases.coming);
                });
            } else {
                ui.label("Failed to load releases");
            }
        });
    }
}
