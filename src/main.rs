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

    let mut graphic_context = GraphicsContext::new();
    let mut my_ui = Ui::new(&mut graphic_context, screen_width(), screen_height());

    loop {
        clear_background(GRAY);

        gui_mq::draw_station(&mut my_ui);
        let my_vec: Option<Vec2> = Some(Vec2::new(0 as f32, 300 as f32));
        let my_string: Cow<'_, str> = Cow::Owned("holllaaaaa".to_string());
        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
            .label("comicon")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
            if ui.button(Vec2::new(260., 70.), "weeb") {
                let (mouse_x, mouse_y) = mouse_position();
                draw_circle(mouse_x, mouse_y, 500., GREEN);
            }
        });
        let button_return = my_ui.button( my_vec,  UiContent::Label(my_string));
        next_frame().await;
    }
}