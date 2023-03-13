use std::borrow::Cow;
use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::NodeIndex;
use petgraph::prelude::StableGraph;
use crate::graph::*;
use crate::route::Route;
use crate::station::Station;
use crate::train::TrainType::*;

use egui::*;

struct StationCoordinates {
    node_index: NodeIndex,
    x: f32,
    y: f32,
}
// EGUI / EFRAME

pub struct TemplateWindow {
    label: String,
}
impl Default for TemplateWindow {
    fn default() -> Self {
       Self {
           label: "RlwyNetSim".to_owned(),
       }
    }
}
impl TemplateWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}
impl eframe::App for TemplateWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

pub struct StationUI {
    name: String,
    n_freight: u32,
    n_lowspeed: u32,
    n_highspeed: u32,
}
impl Default for StationUI {
    fn default() -> Self {
        Self {
            name: "Berlin".to_owned(),
            n_freight: 5,
            n_lowspeed: 5,
            n_highspeed: 5,
        }
    }
}
impl StationUI {
    pub fn make_station(&mut self, ui: &mut egui::Ui) {
        ui.heading("Create Station");

        ui.horizontal(|ui| {
            ui.label("Station Name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.label("Number of Platforms");
        egui::Slider::new(&mut self.n_freight, 0..=100).text("Freight Platforms");
        egui::Slider::new(&mut self.n_lowspeed, 0..=100).text("LowSpeed Platforms");
        egui::Slider::new(&mut self.n_highspeed, 0..=100).text("HighSpeed Platforms");
        ui.label(format!("Your Station: Name '{}', # of platforms: {}", self.name, self.n_highspeed + self.n_lowspeed + self.n_freight));
    }
}
// add the stations to the station_coordinates
pub fn build_station(x_coordinate: f32, y_coordinate: f32)  {

   // let station = station_coordinates {
   //     node_index: ,
   //     x: x_coordinate,
   //     y: y_coordinate,
   // }
   //
   // draw_circle(x, y, 20., RED);

}

