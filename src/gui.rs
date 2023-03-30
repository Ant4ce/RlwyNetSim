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
pub struct StationUI(NodeIndex);

#[derive(Component)]
struct RouteUI(EdgeIndex);

#[derive(Component)]
pub struct Name(String);

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

#[derive(Default)]
pub struct EguiState {
    plat_name: String,
    plat_LowS: u8,
    plat_HighS: u8,
    plat_Freight: u8,
    hand_cursor: bool,
}

//fn ui_add_station(mut commands: Commands, name: Name, pos: Position,
//                  pf_f: PlatformFreight, pf_h: PlatformHighS, pf_l: PlatformLowS,
//                  graph: &mut Arc<RwLock<StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>,
//                  id: &mut u32) {
//    let id = add_station_to_graph(graph, id, name.0.clone(),
//                          vec![(pf_f.0, Freight), (pf_h.0, HighSpeed), (pf_l.0, LowSpeed)]);
//    commands.spawn((name, pos, pf_f, pf_h, pf_l));
//}

pub fn central_ui(mut ctx: EguiContexts, mut commands: Commands,
                  stations: Query<&Name>, mut egui_params: Local<EguiState>) {

    // Set
    if egui_params.hand_cursor {
        ctx.ctx_mut().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

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
        make_station(ui, &mut egui_params, commands);
    });
}


pub fn make_station(ui: &mut egui::Ui, egui_params: &mut Local<EguiState>, mut commands: Commands) {
    ui.heading("Create Station");

    ui.horizontal(|ui| {
        ui.label("Station Name: ");
        ui.text_edit_singleline(&mut egui_params.plat_name);
    });
    ui.label("Number of Platforms");
    ui.add(egui::Slider::new(&mut egui_params.plat_Freight, 0..=100).text("Freight Platforms"));
    ui.add(egui::Slider::new(&mut egui_params.plat_LowS, 0..=100).text("LowSpeed Platforms"));
    ui.add(egui::Slider::new(&mut egui_params.plat_HighS, 0..=100).text("HighSpeed Platforms"));
    ui.label(format!("Your Station: Name '{}', # of platforms: {}", egui_params.plat_name,
                     egui_params.plat_Freight + egui_params.plat_HighS + egui_params.plat_LowS));
    if ui.add(egui::Button::new("Create!")).clicked() {
        commands.spawn(Name(egui_params.plat_name.clone()));
        //ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
        egui_params.hand_cursor = true;
    }

    //return false;

}


//pub fn ui_default_values(mut commands: Commands) {
//    commands.spawn((DefaultStation, Name(String::from("Berlin")),
//                    PlatformFreight(0), PlatformLowS(0), PlatformHighS(0)));
//}
