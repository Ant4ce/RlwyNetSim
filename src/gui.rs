use std::fmt::{Debug, Display, format, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::f32::Vec3;
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
pub struct StationUI;

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
        egui_params.hand_cursor = true;
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
       transform: Transform::from_xyz(window.width()/ 2.0, window.height() /2.0, 0.0),
        ..default()
    });
}

pub const CAMERA_SPEED: f32 = 300.0;

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_2d: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {

    let mut my_camera = camera_2d.single_mut();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0 , 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0 );
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0 );
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0 );
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    my_camera.translation += direction * CAMERA_SPEED * time.delta_seconds();

}
pub fn ui_spawn_station(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    // to pass to the SpriteBundle to indicate the location.
    let window = window_query.get_single().unwrap();
    //Changes the sprite size.
    let my_sprite = Sprite{
        custom_size: Some(Vec2{x: 50.0, y: 50.0}),
        ..default()
    };
    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() /2.0, 0.0),
            sprite: my_sprite,
            texture: asset_server.load("sprites/planets/planet00.png"),
            ..default()
    },
        StationUI,
    ));
    let second_sprite = Sprite{
        custom_size: Some(Vec2{x: 50.0, y: 50.0}),
        ..default()
    };
    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() /2.0 + 100.0, 0.0),
            sprite: second_sprite,
            texture: asset_server.load("sprites/planets/planet01.png"),
            ..default()
        },
         StationUI,
        ));
}


//pub fn ui_default_values(mut commands: Commands) {
//    commands.spawn((DefaultStation, Name(String::from("Berlin")),
//                    PlatformFreight(0), PlatformLowS(0), PlatformHighS(0)));
//}
