pub mod station;
pub mod train;
pub mod route;
pub mod threadpool;
pub mod graph;

use std::sync::{Arc, Mutex};
use crate::train::TrainType;
use crate::station::Station;
use crate::train::Train;
use crate::route::Route;

use petgraph::stable_graph::StableGraph;
use petgraph::dot::Dot;
use petgraph::Undirected;

fn main() {
    
    let mut train_id_counter: u32 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 

    //NEED to USE a DIFFERENT graph type, not GraphMap, is it doesn't allow for mutability in the
    //node weights.
    // see petgraph documentation at: https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html  

    let mut graph = Arc::new((StableGraph::<Station, Route>::new()));

    let test_station = Station::new(&mut station_id_counter, "Geneve".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    let test_2 = Station::new(&mut station_id_counter, "Eindhoven".to_string(), vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);
}