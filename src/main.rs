pub mod station;
pub mod train;
pub mod line;
pub mod edge;

use std::collections::HashMap;
use crate::train::TrainType;
use std::hash::Hash;
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::Dot;
use crate::edge::Edger;

fn main() {
    
    let mut train_id_counter: u16 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 

    // see petgraph documentation at: https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html  
    let mut graph = UnGraphMap::<_, Edger>::new();

    let test_station = station::Station::new(&mut station_id_counter, "ASTAT".to_string(), vec![(2, TrainType::LowSpeed),(3, TrainType::Freight)]); 
    let test_2 = station::Station::new(&mut station_id_counter, "numba2".to_string(), vec![(2, TrainType::LowSpeed), (7, TrainType::HighSpeed)]); 
    
    graph.add_node(&test_station);
    graph.add_node(&test_2);
    graph.add_edge(&test_station, &test_2, Edger::new(&mut route_id_counter, "big_boi_connec".to_string(), test_station.id.clone(), test_2.id.clone()));

    println!("{:?}", Dot::new(&graph));
    println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));
}


