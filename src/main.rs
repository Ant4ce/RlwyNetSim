pub mod station;
pub mod train;
pub mod line;
pub mod edge;

use crate::train::TrainType;
use crate::station::Station;

use petgraph::stable_graph::StableGraph;
use petgraph::dot::Dot;
use petgraph::Undirected;
use crate::edge::Edger;

fn main() {
    
    let mut train_id_counter: u16 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 

    //NEED to USE a DIFFERENT graph type, not GraphMap, is it doesn't allow for mutability in the
    //node weights.
    // see petgraph documentation at: https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html  
    let mut graph = StableGraph::<Station, Edger>::new();

    let test_station = station::Station::new(&mut station_id_counter, "Geneve".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]); 
    let test_2 = station::Station::new(&mut station_id_counter, "Eindhoven".to_string(), vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)]); 
    
    let origin_ind = graph.add_node(test_station);
    let destination_ind = graph.add_node(test_2);
    let connection_ind = graph.add_edge(origin_ind, destination_ind, Edger::new(&mut route_id_counter, "big_boi_connec".to_string(), graph.node_weight(origin_ind).unwrap().id.clone(), graph.node_weight(destination_ind).unwrap().id.clone()));

    
    println!("{:?}", Dot::new(&graph));
    //This part shows that we have mutability of the values inside the graph. So we can change the
    //state of both the tracks(edges) and stations(nodes).
    println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));

    println!("weight of edge: {:?}", graph.edge_weight(connection_ind).unwrap());
    
    let edge_filler: &mut Edger = graph.edge_weight_mut(connection_ind).unwrap();
    edge_filler.name = "my_new_form".to_string();
    println!("weight of edge is now: {:?}", edge_filler);

    println!("{} NEW SECTION {}", "#".repeat(20),"#".repeat(20));

    println!("{:?}", graph.node_weight(origin_ind).unwrap());

    let new_node: &mut Station = graph.node_weight_mut(origin_ind).unwrap();
    new_node.name = "Berlin".to_string(); 
    println!("{:?}", new_node);
    

}


