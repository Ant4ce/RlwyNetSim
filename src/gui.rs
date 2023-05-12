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
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy::render::mesh;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology::LineList;
use bevy::asset::LoadState;

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
pub struct EguiRoute {
    route_name: String,
    bi_directional: bool,
}

#[derive(Default)]
pub struct RouteEndpoints {
    start: Option<NodeIndex>,
    start_coordinates: Vec2,
    end: Option<NodeIndex>,
    end_coordinates: Vec2,
}

#[derive(Resource)]
pub struct MyResources {
    text_field_clicked: bool,
    hand_cursor: bool,
    cursor_world_coordinates: Vec2,
}

#[derive(Resource)]
pub struct show_route(bool);

#[derive(Resource)]
pub struct route_building_materials {
    start: Option<NodeIndex>,
    end: Option<NodeIndex>,
    start_coordinates: Vec2,
    end_coordinates: Vec2,
    build_rail: bool,
    bi_directional_copy: bool,
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
    commands.insert_resource(show_route(false));
    commands.insert_resource(route_building_materials {
        start: None,
        end: None,
        start_coordinates: Vec2{x: 0.0, y: 0.0},
        end_coordinates: Vec2{x: 0.0, y: 0.0},
        build_rail: false,
        bi_directional_copy: false,
    });
}


pub fn central_ui(mut ctx: EguiContexts, mut commands: Commands,
                  stations: Query<&Name, (With<StationComponent>, Without<UnderConstruction>) >,
                  mut egui_params: Local<EguiState>,
                  mut egui_route_params: Local<EguiRoute>,
                  mut resource: ResMut<MyResources>,
                  mut my_route_resource: ResMut<show_route>,
                  mut building_material: ResMut<route_building_materials>,
                  mut graph: ResMut<BevyGraph>,
                  mut route_id: ResMut<RouteIdProvider>,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>) {

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
    if my_route_resource.0 {
        egui::Window::new("Route Creator").show(ctx.ctx_mut(), |ui| {
            ui_route_maker(ui, &mut egui_route_params, my_route_resource, building_material, graph,  route_id);
        });
    }
}

//fn commands_provider(mut commands: Commands) -> Commands{
//    commands
//}

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
    ui.label(format!("Mouse world coordinates: {}, {}", resource.cursor_world_coordinates.x, resource.cursor_world_coordinates.y));
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

/// Egui Window to input the Name and Bi-Directionality of Routes
///
/// This function creates a window to use when creating a route in the simulation
/// it allows the user to input a name as well as choosing uni or bi directional
/// routes. It is an Egui window and is called under the central_ui and so should
/// not be called by anyone else.
pub fn ui_route_maker(ui: &mut egui::Ui,
                      egui_params: &mut Local<EguiRoute>,
                      mut my_route_resource: ResMut<show_route>,
                      mut building_material: ResMut<route_building_materials>,
                      mut graph: ResMut<BevyGraph>,
                      mut route_id: ResMut<RouteIdProvider>) {

    // The following is the format of the window.
    ui.heading("Name Your Route");

    ui.horizontal(|ui| {
        ui.label("Route Name: ");
        ui.text_edit_singleline(&mut egui_params.route_name);
    });
    ui.horizontal(|ui| {
        ui.label("Bi-Directional Route ");
        ui.checkbox(&mut egui_params.bi_directional, "");
        //ui.label(format!("Bi-directional is : {}", egui_params.bi_directional.clone()))
    });
    if ui.add(egui::Button::new("Create Route!")).clicked() {

        //save the user selected option for use in build_rail() function.
        building_material.bi_directional_copy = egui_params.bi_directional.clone();
        // creates the route in the graph.
        let (f, b) =
            graph.0.add_route_to_graph(building_material.start.unwrap(),building_material.end.unwrap(),
                                       &mut route_id.0, egui_params.route_name.clone(), egui_params.bi_directional.clone());
        // tells the build_rail() function that it can build the mesh.
        building_material.build_rail = true;
        //build_rail( commands, meshes, materials, (building_material.start_coordinates, building_material.end_coordinates));

        // tells central_ui() to no longer show the route making window.
        my_route_resource.0 = false;
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    //transform: Transform::from_xyz(window.width()/ 2.0, window.height() /2.0, 0.0),
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 0.0),
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

    // When using a 2d camera, use viewport_to_world_2d instead of world_to_viewport!
    // this is what we used to do.
    // 3d might be world_to_viewport, tested it but seems to only semi-work. It gives coordinates,
    // but these coordinates change more or less depending on whether it is mouse or key movement.
    if let Some(world_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, Vec2::new(cursor.x, cursor.y))) {
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
            let (x, y) = (resource.cursor_world_coordinates.x.clone(), resource.cursor_world_coordinates.y.clone());
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
    mut resource: ResMut<MyResources>,
    station_q: Query<(&Sprite, &Transform, &StationIndex, Entity), With<StationComponent>>,
    buttons: Res<Input<MouseButton>>,
    mut route_stations: Local<RouteEndpoints>,
    mut my_route_resource: ResMut<show_route>,
    mut builing_materials: ResMut<route_building_materials>,
) {
    if buttons.just_pressed(MouseButton::Right){
        println!("Just pressed right button");
        let (old_start, old_end) = (route_stations.start, route_stations.end);
        let (x, y) = (resource.cursor_world_coordinates.x.clone(), resource.cursor_world_coordinates.y.clone());
        for query in station_q.iter() {
            if (((query.1.translation.x - x as f32).pow(2) +
                (query.1.translation.y - y as f32).pow(2)) as f32).sqrt() < query.0.custom_size.unwrap().x / 2.2 {
                println!("x : {:?}, y {:?}", (query.1.translation.x - x).abs(),(query.1.translation.y - x).abs());
                match route_stations.start {
                    None => {route_stations.start = Some(query.2.0); route_stations.start_coordinates = Vec2{x: query.1.translation.x, y: query.1.translation.y};}
                    Some(x) => {
                        route_stations.end = Some(query.2.0);
                        route_stations.end_coordinates = Vec2{x: query.1.translation.x, y: query.1.translation.y};
                        //tells central_ui() to show the route creation window.
                        my_route_resource.0 = true;
                        // saves the locations and NodeIndexes that the user has clicked on, for
                        // use in build_rail() function.
                        builing_materials.start = route_stations.start.clone();
                        builing_materials.end = route_stations.end.clone();
                        builing_materials.start_coordinates = route_stations.start_coordinates.clone();
                        builing_materials.end_coordinates = route_stations.end_coordinates.clone();

                        // resets the state of the resource for the next route creation.
                        (route_stations.start, route_stations.end) = (None, None);
                        route_stations.start_coordinates = Vec2{x: 0.0, y: 0.0};
                        route_stations.end_coordinates = Vec2{x: 0.0, y: 0.0};

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



use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use num_traits::abs;

/// Creates the meshes of the routes.
///
/// This function will check the state of the Simulation and create the
/// meshes for the routes only when the name and bi-directionality state
/// have been provided. It will create Green routes for uni-directional
/// routes and purple ones for bi-directional.
///
/// This function is a system provided to the bevy App. So it should not be
/// called individually by anyone other than bevy itself.
pub fn build_rail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut building_material: ResMut<route_building_materials>,
) {
    // .build_rail is to see if the mesh should be built.
    if building_material.build_rail {
        let (location_1, location_2) = (building_material.start_coordinates, building_material.end_coordinates);
        // calculate the middle point between points, the length and the angle between them.
        let (spawn_point, length, angle_radians) = calculate_middle(location_1, location_2);

        //different visual indication depending on whether bi-directional or not.
        if building_material.bi_directional_copy {
            commands.spawn(MaterialMesh2dBundle {
                // decides the shape of the mesh, its height and width inside the Vec2.
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(25., length)))).into(),
                // decides the spawn location, changes the scale of the mesh and add the angle rotation to the object.
                transform: Transform::from_xyz(spawn_point.x, spawn_point.y, 0.0).with_rotation(Quat::from_rotation_z(angle_radians)),
                // chooses the meshes color.
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            });
        } else {
            // same funcionality as the above if clause.
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(12.5, length)))).into(),
                transform: Transform::from_xyz(spawn_point.x, spawn_point.y, -1.0).with_rotation(Quat::from_rotation_z(angle_radians)),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            });
        }
        // reset the state of the resource to make sure we don't accidentally create the same route multiple times.
        building_material.start = None;
        building_material.end = None;
        building_material.start_coordinates = Vec2{x: 0.0, y: 0.0};
        building_material.end_coordinates = Vec2{x: 0.0, y: 0.0};
        // change to false to make sure the system stops building meshes.
        building_material.build_rail = false;
    }
}

use libm::atan2;

/// Returns tuple of : (Vec2, f32, f32)
///
/// This function calculates the middle point, length and angle between two points given as Vec2's.
/// It gets the angle between the two points by using atan2().
/// # Example:
/// ```
/// let location1 = Vec2{x: 0.7, y: 1.0};
/// let location2 = Vec2{x: -12.0, y: 42.1};
/// let (middle_point, length_between_points, angle) = calculate_middle(location1, location2);
/// ```
fn calculate_middle(location1: Vec2, location2: Vec2) -> (Vec2, f32,f32) {
    // get distance between the two points by using pythagoras.
    let distance_between = (((abs(location2.x - location1.x)).powf(2.0))
        + (abs(location2.y - location1.y)).powf(2.0)).sqrt();
    // calculating middle point location
    let spawn_point: Vec2 = Vec2::new((location1.x + location2.x)/2.0, (location1.y + location2.y)/2.0 );
    // calculate the angle by using the 2-argument arctangent.
    let angle_radians = atan2((spawn_point.x - location1.x).into(), (spawn_point.y - location1.y).into());
    println!("radians angle is : {}", angle_radians);
    // return all the calculated values.
    (spawn_point, distance_between  as f32, (-1.0 * angle_radians as f64) as f32)
}