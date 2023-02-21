use std::fmt::Error;
use std::sync::{Arc, Mutex};
use petgraph::data::DataMap;
use petgraph::graph::Node;
use petgraph::stable_graph::StableGraph;
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use crate::{station::Station, route::Route};
use crate::train::TrainType;

use petgraph::stable_graph::Edges;
use petgraph::{Directed, Incoming, Outgoing};
use petgraph::visit::{EdgeRef, IntoEdgesDirected};

#[derive(Debug)]
pub enum GraphError {
    RemovingStation,
}

pub fn add_station_to_graph(graph: &mut StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                            id: &mut u32, name: String,
                            platforms : Vec<(u8, TrainType)>) -> NodeIndex {

    let new_station = Station::new(id, name, platforms);
    graph.add_node(Arc::new(Mutex::new(new_station)))
}
pub fn add_route_to_graph(graph: &mut StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                          station_a: NodeIndex, station_b: NodeIndex, route_id: &mut u32,
                          name: String) -> EdgeIndex {

    let new_route = Route::new(route_id, name);
    graph.add_edge(station_a, station_b, Arc::new(Mutex::new(new_route)))
}

pub fn remove_station_from_graph(graph: &mut StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                                        index_node: NodeIndex, id_route_counter : &mut u32) -> Result<(), GraphError>  {

    let e_neighbours_incoming = graph.edges_directed(index_node, Incoming);
    let e_neighbours_outgoing = graph.edges_directed(index_node, Outgoing);
    let mut edge_names_and_indexes: Vec<(String, EdgeIndex)> = vec![];

    for x in e_neighbours_incoming {
        let name_route = x.weight().lock().unwrap().name.clone();
        edge_names_and_indexes.push((name_route, x.id()));
    }
    for z in e_neighbours_outgoing {
        let name_route = z.weight().lock().unwrap().name.clone();
        edge_names_and_indexes.push((name_route, z.id()));
    }
    let mut related_routes: Vec<(String, Vec<EdgeIndex>)> = vec![];
    for (name, edge) in edge_names_and_indexes {
        let mut temp_item = related_routes.iter_mut().find(|x| *x.0 == name);
        match temp_item {
            Some(v)  => v.1.push(edge),
            None => related_routes.push((name, vec![edge])),
            _ => panic!("Fails at removing node from graph: remove_station_from_graph(), impossible match default"),
        };
    }
    //TODO: this currently filters out all related routes when there is not exactly 2 of them.
    // This is done because when there was only one route of its kind, it needs to be deleted, as it
    // visits no other station than the one deleted.
    // All other cases we do not yet handle.
    let mut pair_routes = related_routes.iter().filter(|x| x.1.len() == 2);

    for line in pair_routes {
        let route_name = line.0.clone();
        let first_edge = graph.edge_endpoints(line.1[0]).unwrap();
        let second_edge = graph.edge_endpoints(line.1[1]).unwrap();
        let mut new_edge: (NodeIndex, NodeIndex);
        if first_edge.0 == index_node {
            new_edge = (second_edge.0, first_edge.1);
        } else {
            new_edge = (first_edge.0, second_edge.1);
        }
        add_route_to_graph(graph, new_edge.0, new_edge.1, id_route_counter, route_name);
    }

    let removed_node = graph.remove_node(index_node);
    match removed_node {
        Some(x) => Ok(()),
        None => Err(GraphError::RemovingStation),
    }
}

#[cfg(test)]
mod tests {
    use petgraph::data::DataMap;
    use super::*;
    use petgraph::stable_graph::StableGraph;
    use crate::train::TrainType;

    #[test]
    fn adding_station() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>,
            Arc<Mutex<Route>>>::new();

        let test_graph_ind = add_station_to_graph(&mut test_graph, &mut 0,
                                                  String::from("Berlin"),
                                                  vec![(1, TrainType::LowSpeed)]);
        assert_eq!(test_graph.node_weight(test_graph_ind).unwrap().lock().unwrap().name,
                   String::from("Berlin"));
        let mut compare_station = Station::new(&mut 0, String::from("Berlin"),
                         vec![(1, TrainType::LowSpeed)]);
        assert_eq!(*test_graph.node_weight(test_graph_ind).unwrap().lock().unwrap(), compare_station);
    }

    #[test]
    fn adding_route_as_edge() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>,
            Arc<Mutex<Route>>>::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStream"));

        let test_graph_ind_a = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        let test_graph_edge = add_route_to_graph(&mut test_graph, test_graph_ind_a,
                                                 test_graph_ind_b, &mut fake_id,
                                                 String::from("NordStream"));
        assert_eq!(*test_graph.edge_weight(test_graph_edge).unwrap().lock().unwrap().name,
                   String::from("NordStream"));
        assert_eq!(*test_graph.edge_weight(test_graph_edge).unwrap().lock().unwrap(), compare_test_route);
    }
}