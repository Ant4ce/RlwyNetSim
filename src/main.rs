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
use crate::gui_mq::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut station_id_counter: u32 = 0;
    let mut route_id_counter: u32 = 0; 

    //NEED to USE a DIFFERENT graph type, not GraphMap, is it doesn't allow for mutability in the
    //node weights.
    // see petgraph documentation at: https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html  

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

    loop {
        clear_background(GRAY);
        draw_circle(screen_width()/2.,screen_height()/2., 50., RED);
        let circle = Circle::new(screen_width()/2.,screen_height()/2., 50.);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x,mouse_y) = mouse_position();
            let mouse_circ = Circle::new(mouse_x,mouse_y,1.);

            if mouse_circ.overlaps(&circle) {
                draw_circle(screen_width()/2.,screen_height()/2., 200., BLUE);
            }
        }
        next_frame().await;
    }
}