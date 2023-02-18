pub mod station;
pub mod train;
pub mod line;
pub mod edge;
pub mod threadpool;

use std::any::Any;
use crate::train::TrainType;
use crate::station::Station;
use crate::train::Train;

use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use macroquad::prelude::coroutines::wait_seconds;
use petgraph::data::{DataMap, DataMapMut};
use petgraph::stable_graph::StableGraph;
use petgraph::dot::Dot;
use petgraph::Undirected;
use crate::edge::Edger;

fn main() {
    
    let mut train_id_counter: u32 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 

    //NEED to USE a DIFFERENT graph type, not GraphMap, is it doesn't allow for mutability in the
    //node weights.
    // see petgraph documentation at: https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html  
    // THREAD POOL
    // END THREAD POOL

    let mut graph = StableGraph::<Arc<Mutex<Station>>, Edger>::new();

    let test_station = Arc::new(Mutex::new(station::Station::new(&mut station_id_counter, "Geneve".to_string(),
                                                                 vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)])));
    let test_2 = Arc::new(Mutex::new(station::Station::new(&mut station_id_counter, "Eindhoven".to_string(),
                                                                   vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)])));

    let cp_test_station = Arc::clone(&test_station);
    let cp_test_2 = Arc::clone(&test_2);

    let origin_ind = graph.add_node(cp_test_station);
    let destination_ind =graph.add_node(cp_test_2);

    let start_station_id = graph.node_weight(origin_ind).unwrap().lock().unwrap().id.clone();
    let end_station_id = graph.node_weight(destination_ind).unwrap().lock().unwrap().id.clone();

    let connection_ind = graph.add_edge(origin_ind, destination_ind, Edger::new(&mut route_id_counter, "big_boi_connec".to_string(),
                                                start_station_id,
                                                end_station_id));


    //CONCURRENCY test
    let mut handles = vec![];

    let node_start_copy = Arc::clone(&test_station);
    let handle_start = thread::spawn(move || {
        let mut extract1 = node_start_copy.lock().unwrap();
        println!("time has stopped");
        thread::sleep(Duration::from_secs(5));
        extract1.name.push_str("za Warudo");
        println!("Jotaro has moved")
    });
    handles.push(handle_start);

    let node_end_copy = Arc::clone(&test_2);
    let handle_end = thread::spawn(move || {
        let mut extract2 = node_end_copy.lock().unwrap();
        extract2.name.push_str("Dio moves in time thread");
        println!("Dio has moved");
    });
    handles.push(handle_end);

    //for Xer in 0..10 {
    //    let node_copy_1 = Arc::clone(&test_station);
    //    let handle = thread::spawn(move || {
    //        let mut extract = node_copy_1.lock().unwrap();
    //        extract.name.push_str("world");
    //    });
    //    handles.push(handle);
    //}
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result start station: {:?}, the end station: {:?}", graph.node_weight(origin_ind), graph.node_weight(destination_ind));
    
    //println!("{:?}", Dot::new(&graph));
    //This part shows that we have mutability of the values inside the graph. So we can change the
    //state of both the tracks(edges) and stations(nodes).
    //println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));

    //println!("weight of edge: {:?}", graph.edge_weight(connection_ind).unwrap());
    
    //let edge_filler: &mut Edger = graph.edge_weight_mut(connection_ind).unwrap();
    //edge_filler.name = "my_new_form".to_string();
    //println!("weight of edge is now: {:?}", edge_filler);
    /*
    println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));

    let new_node: &mut Station = graph.node_weight_mut(origin_ind).unwrap();
    new_node.name = "Berlin".to_string(); 
    println!("{:?}", new_node);

    let test_train = train::Train::new(&mut train_id_counter,"X_model".to_string(), true, TrainType::LowSpeed, 10, "S-Bahn".to_string());
    new_node.trains.push(test_train);
    println!("{:?}", new_node);
    println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));
    println!("{:?}", Dot::new(&graph));
    */

}


