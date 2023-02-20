use std::sync::{Arc, Mutex};
use petgraph::data::DataMap;
use petgraph::graph::Node;
use petgraph::stable_graph::StableGraph;
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use crate::{station::Station, route::Route};
use crate::train::TrainType;

use petgraph::stable_graph::Edges;
use petgraph::Directed;
use petgraph::visit::EdgeRef;

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
                                        index_node: NodeIndex)  {

    let e_neighbours: Edges<Arc<Mutex<Route>>, Directed> = graph.edges(index_node);
    let mut hold_names_indexes: Vec<(String, EdgeIndex)> = vec![];

    for x in e_neighbours {
        let a = x.clone().weight();
        let name_route = a.lock().unwrap().name.clone();
        hold_names_indexes.push((name_route, x.id()));
    }
    let mut related_routes: Vec<(String, Vec<EdgeIndex>)> = vec![];
    for b in hold_names_indexes {
        let (name, edge) = b;
        let mut temp_item = related_routes.iter_mut().find(|x| *x.0 == name);
        match temp_item {
            Some(v)  => v.1.push(edge),
            None => related_routes.push((name, vec![edge])),
            _ => continue,
        };
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