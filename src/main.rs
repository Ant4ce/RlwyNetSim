pub mod station;
pub mod train;
pub mod route;
pub mod threadpool;
pub mod graph;
pub mod gui_mq;

use std::sync::{Arc, Mutex, RwLock};
use crate::train::{TrainRegister, TrainType};
use crate::station::Station;
use crate::train::{Location};
use crate::route::Route;

use petgraph::stable_graph::StableGraph;
use crate::graph::*;
use crate::threadpool::ThreadPool;

use macroquad::prelude::*;
use crate::miniquad::GraphicsContext;
use std::borrow::Cow;
use macroquad::ui::UiContent;
use macroquad::ui::*;
use macroquad::hash;
use crate::widgets::Group;


use crate::gui_mq::window_conf;
use crate::train::TrainType::{Freight, HighSpeed, LowSpeed};

#[macroquad::main(window_conf)]
async fn main() {

    let mut station_id_counter: u32 = 0;
    let mut route_id_counter: u32 = 0; 


    let mut graph = Arc::new(RwLock::new(StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new()));

    let start_node = add_station_to_graph(&mut graph, &mut station_id_counter, "Geneve".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    let middle_station = add_station_to_graph(&mut graph, &mut station_id_counter, "Paris".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    let end_node= add_station_to_graph(&mut graph, &mut station_id_counter, "Eindhoven".to_string(), vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);

    add_route_to_graph(&mut graph, start_node, middle_station, &mut route_id_counter, String::from("S1"), true);
    add_route_to_graph(&mut graph, middle_station, end_node, &mut route_id_counter, String::from("S1"), true);

    let mut register = TrainRegister::new(String::from("S-Bahn fleet"));
    register.add_train(TrainType::Freight, Location::NodeTypeIndex(start_node), String::from("S1"), true, "Passenger".to_string());
    register.add_train(TrainType::Freight, Location::NodeTypeIndex(middle_station), String::from("S1"), true, "Passenger".to_string());

    let pool = ThreadPool::new(4);

    for train in &register.train_list {
        let arc_graph = Arc::clone(&graph);
        let train_new = Arc::clone(&train);
        pool.execute(move || train_new.lock().unwrap().move_forward(&arc_graph));
    }

    let mut station_name = String::new();
    let mut data = gui_mq::Data::new();
    let (mut number0, mut number1, mut number2) = (0f32, 0f32, 0f32);

    loop {
        clear_background(GRAY);

        widgets::Window::new(hash!(), vec2(1200., 50.), vec2(300., 300.))
            .label("Station Creation")
            .ui(&mut *root_ui(), |ui| {
                ui.tree_node(hash!(), "Input", |ui| {
                    ui.label(None, "Station name:");
                    ui.input_text(hash!(), "", &mut station_name);
                });
                // The slider's input value will get rounded down.
                ui.tree_node(hash!(), "Platforms", |ui| {
                    ui.label(None, "LowSpeed");
                    ui.slider(hash!(), "[0..100]", 0f32..100f32, &mut number0);
                    ui.label(None, "HighSpeed");
                    ui.slider(hash!(), "[0..100]", 0f32..100f32, &mut number1);
                    ui.label(None, "Freight");
                    ui.slider(hash!(), "[0..100]", 0f32..100f32, &mut number2);
                });
                if ui.button(None, "Create Station")  {
                    let my_node = add_station_to_graph(&mut graph, &mut station_id_counter, station_name.clone(),
                                                       vec![(number0 as u8, LowSpeed), (number1 as u8, HighSpeed), (number2 as u8, Freight)]);
                    data.inventory.push(format!("name: {:?}, NodeIndex: {:?}",
                                                graph.read().unwrap().node_weight(my_node).unwrap().lock().unwrap().name,
                                                my_node));
                };
            });
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(400., 700.))
            .label("My Stations")
            .ui(&mut *root_ui(), |ui| {
                data.inventory(ui);
            });
        next_frame().await;
    }
}