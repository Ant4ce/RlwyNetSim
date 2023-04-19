use std::fmt::{Debug, Display, format, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use num_traits::pow::Pow;
use bevy::ecs::query::WorldQuery;
use petgraph::stable_graph::{StableGraph, NodeIndex, EdgeIndex};
use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::f32::Vec3;
use bevy::render::camera::RenderTarget;
use petgraph::graph::node_index;
use crate::graph::Graph;
use crate::route::Route;
use crate::station::Station;
use crate::train::TrainType::{LowSpeed, HighSpeed, Freight};

#[derive(Component)]
struct DefaultStation;

#[derive(Component)]
struct DefaultRoute;

#[derive(Component)]
pub struct UnderConstruction;

#[derive(Component)]
struct RouteUI(EdgeIndex);

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct StationIndex(NodeIndex);

#[derive(Component)]
pub struct StationComponent;

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

#[derive(Component)]
pub struct Platforms {
    low_speed: PlatformLowS,
    high_speed: PlatformHighS,
    freight: PlatformFreight
}

#[derive(Bundle)]
pub struct StationBundle{
    name: Name,

    #[bundle]
    platforms: Platforms,

    station: StationComponent
}

#[derive(Default)]
pub struct EguiState {
    plat_name: String,
    plat_LowS: u8,
    plat_HighS: u8,
    plat_Freight: u8,
}

#[derive(Default)]
pub struct RouteEndpoints {
    start: Option<NodeIndex>,
    end: Option<NodeIndex>,
}

#[derive(Resource)]
pub struct MyResources {
    text_field_clicked: bool,
    hand_cursor: bool,
    cursor_world_coordinates: Vec2,
}

#[derive(Resource)]
pub struct BevyGraph(Graph);

#[derive(Resource)]
pub struct StationIdProvider(u32);

#[derive(Resource)]
pub struct RouteIdProvider(u32);

// This adds our Resources to our World component. Resources are pieces of data that
// can be shared by multiple different parts of the bevy code.
pub fn instantiate_resources(mut commands: Commands) {
    commands.insert_resource(MyResources{
        text_field_clicked: false,
        hand_cursor: false,
        cursor_world_coordinates: Vec2::ZERO,
    });
    commands.insert_resource(BevyGraph(Graph::new()));
    commands.insert_resource(StationIdProvider(0));
    commands.insert_resource(RouteIdProvider(0));
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
                  stations: Query<&Name, (With<StationComponent>, Without<UnderConstruction>) >, mut egui_params: Local<EguiState>,
                  mut resource: ResMut<MyResources>) {

    // Set cursor type
    if resource.hand_cursor {
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
        make_station(ui, &mut egui_params, commands, resource);
    });
}


pub fn make_station(ui: &mut egui::Ui, egui_params: &mut Local<EguiState>,
                    mut commands: Commands, mut resource: ResMut<MyResources>) {
    ui.heading("Create Station");

    ui.horizontal(|ui| {
        ui.label("Station Name: ");
        let a_response = ui.text_edit_singleline(&mut egui_params.plat_name);
        if a_response.clicked() {
            resource.text_field_clicked = true;
        } else if a_response.clicked_elsewhere() {
            resource.text_field_clicked = false;
        }
    });
    ui.label("Number of Platforms");
    ui.add(egui::Slider::new(&mut egui_params.plat_Freight, 0..=100).text("Freight Platforms"));
    ui.add(egui::Slider::new(&mut egui_params.plat_LowS, 0..=100).text("LowSpeed Platforms"));
    ui.add(egui::Slider::new(&mut egui_params.plat_HighS, 0..=100).text("HighSpeed Platforms"));
    ui.label(format!("Your Station: Name '{}', # of platforms: {}", egui_params.plat_name,
                     egui_params.plat_Freight + egui_params.plat_HighS + egui_params.plat_LowS));
    if ui.add(egui::Button::new("Create!")).clicked() {
        commands.spawn((StationBundle{name: Name(egui_params.plat_name.clone()),
            platforms: Platforms{
                low_speed: PlatformLowS(egui_params.plat_LowS.clone()),
                high_speed: PlatformHighS(egui_params.plat_HighS.clone()),
                freight: PlatformFreight(egui_params.plat_Freight.clone())
            },
            station: StationComponent //inside Bundle
        },
            UnderConstruction //Component outside Bundle
        ));
        resource.hand_cursor = true;
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
       transform: Transform::from_xyz(window.width()/ 2.0, window.height() /2.0, 0.0),
        ..default()
    });
}

// Gets the position of the cursor and converts it into the world coordinates.
// These world coordinates can be used to spawn objects based on cursor position
// and are saved in resources.
pub fn cursor_location_in_world(
    window_query: Query<&Window>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut resource: ResMut<MyResources>)
{
    let (camera, camera_transform) = query_camera.single();
    let window = window_query.single();

    if let Some(world_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        //println!("world coordinates: {} , {}", world_position.x, world_position.y);
        resource.cursor_world_coordinates = world_position;
    }

}

pub const CAMERA_SPEED: f32 = 300.0;

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_2d: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    resource: Res<MyResources>,
) {
    if resource.text_field_clicked == false {
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

}


pub fn ui_spawn_station(
    mut commands: Commands,
    mut ctx: EguiContexts,
    asset_server: Res<AssetServer>,
    mut resource: ResMut<MyResources>,
    buttons: Res<Input<MouseButton>>,
    station_info: Query<(&Name, &Platforms, Entity), With<UnderConstruction>>, //TODO: make query to get name and platforms
    mut graph: ResMut<BevyGraph>,
    mut next_id: ResMut<StationIdProvider>
) {
    if resource.hand_cursor == true {
        if buttons.just_pressed(MouseButton::Left) {
            let the_station = station_info.get_single().unwrap();

            //Changes the sprite size.
            let my_sprite = Sprite{
                custom_size: Some(Vec2{x: 50.0, y: 50.0}),
                ..default()
            };
            //EXPLANATION: Whats getting unwrapped here?!
            // Well as you can see we do the_station.0.0.clone() -> This is because
            // we Queried for Name and Platforms, here we needed *Name* which is the first element, which is the first zero/0.
            // The next zero is the *String* that is stored inside our Name component. Finally we clone it because we
            // cannot move the name value out. And there you go! That's how you get convoluted / beautiful code.
            // In pseudocode, this equates to Query<&Name, &Platforms>.Name.unwrapString.clone()
            //
            //Similar unwrapping is needed for the Platforms, as the Platform Components wrap a u8 number
            let node_index = graph.0.add_station_to_graph(&mut next_id.0, the_station.0.0.clone(), vec![
                (the_station.1.low_speed.0, LowSpeed),
                (the_station.1.high_speed.0, HighSpeed),
                (the_station.1.freight.0, Freight) ]);
            commands.entity(the_station.2).remove::<UnderConstruction>();
            let (x, y) = (resource.cursor_world_coordinates.x.clone(),
                                resource.cursor_world_coordinates.y.clone());
            commands.entity(the_station.2).insert(
                (StationIndex(node_index.clone()), SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    sprite: my_sprite,
                    texture: asset_server.load("sprites/planets/planet00.png"),
                    ..default()
                })
            );
            ctx.ctx_mut().set_cursor_icon(egui::CursorIcon::Default);
            resource.hand_cursor = false;
            println!("{:?}", node_index);
        }
    }
}

pub fn route_making(
    //mut commands: Commands,
    resource: Res<MyResources>,
    station_q: Query<(&Sprite, &Transform, &StationIndex, Entity), With<StationComponent>>,
    buttons: Res<Input<MouseButton>>,
    mut route_stations: Local<RouteEndpoints>,
    mut graph: ResMut<BevyGraph>,
    mut route_id: ResMut<RouteIdProvider>,
) {
    if buttons.just_pressed(MouseButton::Right){
        let (old_start, eld_end) = (route_stations.start, route_stations.end);
        let (x, y) = (resource.cursor_world_coordinates.x.clone(), resource.cursor_world_coordinates.y.clone());
        for query in station_q.iter() {
            if (((query.1.translation.x - x as f32).pow(2) +
                (query.1.translation.y - y as f32).pow(2)) as f32).sqrt() < query.0.custom_size.unwrap().x / 2.2 {
                println!("x : {:?}, y {:?}", (query.1.translation.x - x).abs(),(query.1.translation.y - x).abs());
                match route_stations.start {
                    None => {route_stations.start = Some(query.2.0);}
                    Some(x) => {
                        route_stations.end = Some(query.2.0);
                        let (f, b) =
                            graph.0.add_route_to_graph(route_stations.start.unwrap(),route_stations.end.unwrap(),
                                                       &mut route_id.0, String::from("HyperLane"), true);
                        (route_stations.start, route_stations.end) = (None, None);

                        println!("{:?}, {:?}",f, b);
                        break
                    }
                }
            }
        }
        match old_start {
            Some(T) => {
                match route_stations.start {
                    Some(X) => {
                        if T == X {
                            route_stations.start = None;
                        }
                    }
                    None => (),
                }
            }
            None => ()
        }
    }
}
