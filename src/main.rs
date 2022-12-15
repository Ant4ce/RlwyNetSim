pub mod station;
pub mod train;
pub mod line;

use std::collections::HashMap;
use crate::train::TrainType;
use std::hash::Hash;
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::Dot;

fn main() {
    
    let mut train_id_counter: u16 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 

    let mut graph = UnGraphMap::<_, ()>::new();

    //let mut station_identification: HashMap<u32, station::Station> = HashMap::new();

    // when calling the constructor of station be sure to 
    // insert the returned station Object into the HashMap.


    let test_station = station::Station::new(&mut station_id_counter, "ASTAT".to_string(), vec![(2, TrainType::LowSpeed),(3, TrainType::Freight)]); 
    let test_2 = station::Station::new(&mut station_id_counter, "numba2".to_string(), vec![(2, TrainType::LowSpeed), (7, TrainType::HighSpeed)]); 
    
    graph.add_node(&test_station);
    graph.add_node(&test_2);
    graph.add_edge(&test_station, &test_2, ());

    println!("{:?}", Dot::new(&graph));
}


