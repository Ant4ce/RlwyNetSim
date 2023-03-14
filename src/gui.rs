use std::borrow::Cow;
use std::fmt::{Debug, Display, format, Formatter};
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
    stations: Vec<StationUI>,
    current_station: StationUI,
}
impl Default for TemplateWindow {
    fn default() -> Self {
       Self {
           label: "RlwyNetSim".to_owned(),
           stations: vec![],
           current_station: StationUI::default(),
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
        let Self { label, stations, current_station} = self;

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
                ui.menu_button("Station Creator", |ui| {
                    if ui.button("Make Station").clicked() {
                        stations.push(StationUI::default());
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            egui::ScrollArea::vertical().max_width(f32::INFINITY).auto_shrink([false;2]).show(ui, |ui| {
                for stat in stations.clone() {
                    ui.label(format!("{}", stat));
                }
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

            if stations.len() < 1 {
                stations.push(StationUI::default());
            }
            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            ui.label(format!("Your latest Station name: {}, the total  number of station: {}",
                             &stations.last().unwrap().name, stations.len()));
            egui::Window::new("Station Creation").show(ctx, |ui| {
                let bool_result = current_station.make_station(ui);
                if bool_result {
                    stations.push(current_station.clone());
                }
                //match the_station {
                //    Some(x) => stations.push(x.clone()),
                //    None => (),
                //}
            });
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
#[derive(PartialEq, Clone, Debug)]
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
impl Display for StationUI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" ,self.name)
    }
}
impl StationUI {
    pub fn new(name: String, n_f: u32, n_l: u32, n_h: u32) -> StationUI {
        StationUI {
            name: name,
            n_freight: n_f,
            n_lowspeed: n_l,
            n_highspeed: n_h,
        }
    }
    pub fn make_station(&mut self, ui: &mut egui::Ui) -> bool {
        ui.heading("Create Station");

        ui.horizontal(|ui| {
            ui.label("Station Name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.label("Number of Platforms");
        ui.add(egui::Slider::new(&mut self.n_freight, 0..=100).text("Freight Platforms"));
        ui.add(egui::Slider::new(&mut self.n_lowspeed, 0..=100).text("LowSpeed Platforms"));
        ui.add(egui::Slider::new(&mut self.n_highspeed, 0..=100).text("HighSpeed Platforms"));
        ui.label(format!("Your Station: Name '{}', # of platforms: {}", self.name, self.n_highspeed + self.n_lowspeed + self.n_freight));
        if ui.add(egui::Button::new("Create!")).clicked() {
            return true;
        }
        return false;

    }
}

