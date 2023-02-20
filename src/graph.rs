use std::sync::{Arc, Mutex};
use petgraph::data::DataMap;
use petgraph::stable_graph::StableGraph;
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use crate::{station::Station, route::Route};
use crate::train::TrainType;

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

        let mut fake_id:u32 = 0;
        let test_station = Station::new(&mut fake_id, String::from("Berlin"),
                                        vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind = add_station_to_graph(&mut test_graph, test_station);
        assert_eq!(test_graph.node_weight(test_graph_ind).unwrap().lock().unwrap().name,
                   String::from("Berlin"));
        let mut compare_station = Station::new(&mut (fake_id-1), String::from("Berlin"),
                         vec![(1, TrainType::LowSpeed)]);
        assert_eq!(*test_graph.node_weight(test_graph_ind).unwrap().lock().unwrap(), compare_station);
    }

    #[test]
    fn adding_route_as_edge() {
        let mut test_graph = StableGraph::<Arc<Mutex<Station>>,
            Arc<Mutex<Route>>>::new();

        let mut fake_id:u32 = 0;
        let test_station_a = Station::new(&mut fake_id, String::from("Berlin"),
                                        vec![(1, TrainType::LowSpeed)]);
        let test_station_b = Station::new(&mut fake_id, String::from("Moscow"),
                                          vec![(1, TrainType::LowSpeed), (1, TrainType::HighSpeed)]);
        let test_route = Route::new(&mut fake_id, String::from("NordStream"),
                                    test_station_a.id.clone(), test_station_b.id.clone());
        let compare_test_route = test_route.clone();
        let test_graph_ind_a = add_station_to_graph(&mut test_graph, test_station_a);
        let test_graph_ind_b = add_station_to_graph(&mut test_graph, test_station_b);
        let test_graph_edge = add_route_to_graph(&mut test_graph, test_graph_ind_a,
                                                 test_graph_ind_b, test_route);
        assert_eq!(*test_graph.edge_weight(test_graph_edge).unwrap().lock().unwrap().name,
                   String::from("NordStream"));
        assert_eq!(*test_graph.edge_weight(test_graph_edge).unwrap().lock().unwrap(), compare_test_route);
    }
}