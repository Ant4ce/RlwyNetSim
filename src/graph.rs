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

#[derive(Debug, PartialEq)]
pub enum GraphError {
    RemovingStation,
}

pub fn add_station_to_graph(graph: &mut StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                            id: &mut u32, name: String,
                            platforms : Vec<(u8, TrainType)>) -> NodeIndex {

    let new_station = Station::new(id, name, platforms);
    graph.add_node(Arc::new(Mutex::new(new_station)))
}
//TODO: make sure that one station can not have two INCOMING routes of the same name without an
// outgoing route and the same for two OUTGOING routes without an incoming route.
pub fn add_route_to_graph(graph: &mut StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                          station_a: NodeIndex, station_b: NodeIndex, route_id: &mut u32,
                          name: String, bidirectional: bool)  {
    let (mut name_f, mut name_b) = (name.clone(), name.clone());
    name_f.push('f');
    name_b.push('b');

    let new_route = Route::new(route_id, name_f);
    let new_route_reverse_direction = Route::new(route_id, name_b);

    graph.add_edge(station_a, station_b, Arc::new(Mutex::new(new_route)));
    if bidirectional == true {
        graph.add_edge(station_b, station_a, Arc::new(Mutex::new(new_route_reverse_direction)));
    }
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
        add_route_to_graph(graph, new_edge.0, new_edge.1, id_route_counter, route_name, true);
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
        let compare_test_route = Route::new(&mut 2, String::from("NordStreamf"));
        let compare_test_route_reverse = Route::new(&mut 3, String::from("NordStreamb"));

        let test_graph_ind_a = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        add_route_to_graph(&mut test_graph, test_graph_ind_a,
                                                 test_graph_ind_b, &mut fake_id,
                                                 String::from("NordStream"), true);
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStreamf"));
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap(), compare_test_route);

        // testing that reverse direction edge was created.
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStreamb"));
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap(), compare_test_route_reverse);
    }
    #[test]
    fn adding_uni_directional_edge() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>,
            Arc<Mutex<Route>>>::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStream2f"));

        let test_graph_ind_a = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        add_route_to_graph(&mut test_graph, test_graph_ind_a,
                           test_graph_ind_b, &mut fake_id,
                           String::from("NordStream2"), false);
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStream2f"));
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap(), compare_test_route);

    }
    #[test]
    #[should_panic]
    fn panicking_adding_uni_directional_edge() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>,
            Arc<Mutex<Route>>>::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStream2f"));
        let compare_test_route_reverse = Route::new(&mut 2, String::from("NordStream2b"));

        let test_graph_ind_a = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        add_route_to_graph(&mut test_graph, test_graph_ind_a,
                           test_graph_ind_b, &mut fake_id,
                           String::from("NordStream2"), false);
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStream2f"));
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap(), compare_test_route);

        //check that reverse direction doesn't exist, this should cause a panic.
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStream2b"));
        assert_eq!(*(test_graph.edge_weight(test_graph.find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap(), compare_test_route);

    }
    #[test]
    fn removing_a_route_from_graph() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();

        let mut fake_id: u32 = 0;
        let test_station_1 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                        String::from("Warsaw"),
                                        vec![(1, TrainType::Freight), (2, TrainType::LowSpeed)]);

        let test_station_2 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Madrid"),
                                                  vec![(1, TrainType::HighSpeed), (2, TrainType::LowSpeed)]);
        let test_station_3 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Paris"),
                                                  vec![(1, TrainType::Freight), (2, TrainType::HighSpeed)]);

        let test_edge_1 = add_route_to_graph(&mut test_graph, test_station_1, test_station_3, &mut fake_id, String::from("waris"), true);
        let test_edge_2 = add_route_to_graph(&mut test_graph, test_station_3, test_station_2, &mut fake_id, String::from("waris"), true);

        assert_eq!(true, test_graph.contains_node(test_station_3));

        let removal = remove_station_from_graph(&mut test_graph, test_station_3, &mut fake_id);
        // add the assertequal, to test wether removal worked.

        assert_eq!(Ok(()), removal);
        assert_eq!(test_graph.edges_connecting(test_station_1, test_station_2).count(), 2);
        assert_eq!(false, test_graph.contains_node(test_station_3));

    }
    #[test]
    fn  test_major_station_removal() {

        let mut test_graph = StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();

        let mut fake_id: u32 = 0;
        let test_station_1 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Warsaw"),
                                                  vec![(1, TrainType::Freight), (2, TrainType::LowSpeed)]);
        let test_station_2 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Madrid"),
                                                  vec![(1, TrainType::HighSpeed), (2, TrainType::LowSpeed)]);
        let test_station_3 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Paris"),
                                                  vec![(1, TrainType::Freight), (3, TrainType::HighSpeed)]);
        let test_station_4 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("London"),
                                                  vec![(2, TrainType::Freight), (4, TrainType::HighSpeed)]);
        let test_station_5 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Milan"),
                                                  vec![(7, TrainType::Freight), (5, TrainType::HighSpeed)]);
        let test_station_6 = add_station_to_graph(&mut test_graph, &mut fake_id,
                                                  String::from("Pontsarn"),
                                                  vec![(3, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);

        let test_edge_1 = add_route_to_graph(&mut test_graph, test_station_1, test_station_3, &mut fake_id, String::from("waris"), false);
        let test_edge_2 = add_route_to_graph(&mut test_graph, test_station_3, test_station_2, &mut fake_id, String::from("waris"), false);
        let test_edge_3 = add_route_to_graph(&mut test_graph, test_station_4, test_station_3, &mut fake_id, String::from("Loilan"), false);
        let test_edge_4 = add_route_to_graph(&mut test_graph, test_station_3, test_station_5, &mut fake_id, String::from("Loilan"), false);
        let test_edge_5 = add_route_to_graph(&mut test_graph, test_station_3, test_station_6, &mut fake_id, String::from("Parn"), false);

        assert_eq!(true , test_graph.contains_node(test_station_3));

        let removal = remove_station_from_graph(&mut test_graph, test_station_3, &mut fake_id);
        assert_eq!(Ok(()), removal);
        assert_eq!(test_graph.edges_connecting(test_station_1, test_station_2).count(), 1);
        assert_eq!(test_graph.edges_connecting(test_station_4, test_station_5).count(), 1);
        assert_eq!(test_graph.edges(test_station_6).count(), 0);
        assert_eq!(false, test_graph.contains_node(test_station_3));
    }
}