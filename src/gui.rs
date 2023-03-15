use std::fmt::{Debug, Display, format, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use petgraph::prelude::StableGraph;
use crate::graph::add_station_to_graph;
use crate::route::Route;
use crate::station::Station;
use crate::train::TrainType::{Freight, LowSpeed, HighSpeed};

#[derive(Component)]
struct DefaultStation;

#[derive(Component)]
struct DefaultRoute;

#[derive(Component)]
struct StationUI(NodeIndex);

#[derive(Component)]
struct RouteUI(EdgeIndex);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Endpoints(Position, Position);

#[derive(Component)]
struct PlatformFreight(u8);

#[derive(Component)]
struct PlatformHighS(u8);

#[derive(Component)]
struct PlatformLowS(u8);

fn ui_add_station(mut commands: Commands, name: Name, pos: Position,
                  pf_f: PlatformFreight, pf_h: PlatformHighS, pf_l: PlatformLowS,
                  graph: &mut Arc<RwLock<StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>,
                  id: &mut u32) {
    let id = add_station_to_graph(graph, id, name.0.clone(),
                          vec![(pf_f.0, Freight), (pf_h.0, HighSpeed), (pf_l.0, LowSpeed)]);
    commands.spawn((name, pos, pf_f, pf_h, pf_l));
}

pub fn central_ui(mut ctx: EguiContexts, mut command: Commands, stations: Query<&Name, With<StationUI>>) {
    // Examples of how to create different panels and windows.
    // Pick whichever suits you.
    // Tip: a good default choice is to just keep the `CentralPanel`.
    // For inspiration and more examples, go to https://emilk.github.io/egui
    let mut current_station: StationUI = StationUI::default();

    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
    egui::TopBottomPanel::top("top_panel").show(ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
            });
        });
    });

    egui::SidePanel::left("side_panel").show(ctx.ctx_mut(), |ui| {
        ui.heading("Side Panel");

        egui::ScrollArea::vertical().max_width(f32::INFINITY).auto_shrink([false;2])
            .show(ui, |ui| {
            for stat in &stations {
                ui.label(format!("{:?}", stat.0));
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

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's

        ui.heading("eframe template");
        ui.hyperlink("https://github.com/emilk/eframe_template");
        ui.add(egui::github_link_file!(
            "https://github.com/emilk/eframe_template/blob/master/",
            "Source code."
        ));
        egui::warn_if_debug_build(ui);
    });
    egui::Window::new("Station Creation").show(ctx.ctx_mut(), |ui| {
        let bool_result = make_station(ui);
        if bool_result {
            command.spawn((Station))
        }
    });
}

//#[derive(PartialEq, Clone, Debug)]
//pub struct StationUI {
//    name: String,
//    n_freight: u32,
//    n_lowspeed: u32,
//    n_highspeed: u32,
//}
//impl Default for StationUI {
//    fn default() -> Self {
//        Self {
//            name: "Berlin".to_owned(),
//            n_freight: 5,
//            n_lowspeed: 5,
//            n_highspeed: 5,
//        }
//    }
//}
pub fn make_station(query_name: Query<&Name, With<DefaultStation>>, query_pf_f: Query<&PlatformFreight, With<DefaultStation>>,
                    query_pf_l: Query<&PlatformLowS, With<DefaultStation>>, query_pf_h: Query<&PlatformHighS, With<DefaultStation>>,
                    ui: &mut egui::Ui) -> bool {
    ui.heading("Create Station");

    ui.horizontal(|ui| {
        ui.label("Station Name: ");
        ui.text_edit_singleline(&mut query_name);
    });
    ui.label("Number of Platforms");
    ui.add(egui::Slider::new(&mut query_pf_f, 0..=100).text("Freight Platforms"));
    ui.add(egui::Slider::new(&mut query_pf_l, 0..=100).text("LowSpeed Platforms"));
    ui.add(egui::Slider::new(&mut query_pf_h, 0..=100).text("HighSpeed Platforms"));
    ui.label(format!("Your Station: Name '{}', # of platforms: {}", query_name,
                     query_pf_h.0 + query_pf_l.0+ query_pf_f.0));
    if ui.add(egui::Button::new("Create!")).clicked() {
        return true;
    }
    return false;

}


pub fn ui_default_values(mut commands: Commands) {
    commands.spawn((DefaultStation, Name(String::from("Berlin")),
                    PlatformFreight(0), PlatformLowS(0), PlatformHighS(0)));
}
