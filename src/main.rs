pub mod station;
pub mod train;
pub mod route;
pub mod threadpool;
pub mod graph;
pub mod gui;

use std::sync::{Arc, Mutex, RwLock};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::train::{TrainRegister, TrainType};
use crate::station::Station;
use crate::train::{Location};
use crate::route::Route;

use petgraph::stable_graph::StableGraph;
use crate::graph::*;
use crate::threadpool::ThreadPool;

use crate::gui::*;


use crate::train::TrainType::{Freight, HighSpeed, LowSpeed};

fn main() /* -> Result<(), eframe::Error>*/ {

    let mut station_id_counter: u32 = 0;
    let mut route_id_counter: u32 = 0; 


    let mut graph = Graph::new();

    let start_node = graph.add_station_to_graph(&mut station_id_counter, "Geneve".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    let middle_station = graph.add_station_to_graph(&mut station_id_counter, "Paris".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    let end_node= graph.add_station_to_graph(&mut station_id_counter, "Eindhoven".to_string(), vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);

    graph.add_route_to_graph( start_node, middle_station, &mut route_id_counter, String::from("S1"), true);
    graph.add_route_to_graph( middle_station, end_node, &mut route_id_counter, String::from("S1"), true);

    let mut register = TrainRegister::new(String::from("S-Bahn fleet"));
    register.add_train(TrainType::Freight, Location::NodeTypeIndex(start_node), String::from("S1"), true, "Passenger".to_string());
    register.add_train(TrainType::Freight, Location::NodeTypeIndex(middle_station), String::from("S1"), true, "Passenger".to_string());

    let pool = ThreadPool::new(4);

    for train in &register.train_list {
        let arc_graph = Arc::clone(&graph.graph);
        let train_new = Arc::clone(&train);
        pool.execute(move || train_new.lock().unwrap().move_forward(&arc_graph));
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        //.add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(instantiate_resources)
        .add_startup_system(create_triangle)
        .insert_resource(Msaa::Sample4)
        .add_system(central_ui)
        .add_system(move_camera)
        .add_system(cursor_location_in_world)
        .add_system(ui_spawn_station)
        //.add_system(route_making)
        .run();
}
