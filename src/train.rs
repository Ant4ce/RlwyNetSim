use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use petgraph::stable_graph::{NodeIndex, EdgeIndex, EdgeReference};
use petgraph::visit::EdgeRef;
use crate::station::Station;
use crate::route::Route;

#[derive(Debug, PartialEq)]
pub struct Train {
    id: u32,
    train_type: TrainType,
    location: Location,
    route_name: String,
    dir_forward: bool,
    model: String,
}

pub struct TrainRegister {
    name: String,
    next_train_id: u32,
    train_list: Vec<Train>
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TrainType {
    LowSpeed,
    Freight,
    HighSpeed,
}

impl Train {
    fn new(id: &mut u32, train_type: TrainType, location: Location,
               route_name: String, dir_forward: bool, model: String) -> Train {

        let new_train = Train {
            id: id.clone(),
            train_type,
            location,
            route_name,
            dir_forward,
            model: "Passenger".to_string(),
        };
        *id += 1;
        new_train
    }
    pub fn move_forward(&mut self, graph: &petgraph::stable_graph::StableGraph
    <Arc<Mutex<Station>>, Arc<Mutex<Route>>>) {
        let (mut route_forward_name, mut route_backward_name) = (self.route_name.clone(), self.route_name.clone());
        route_forward_name.push('f');
        route_backward_name.push('b');
        match &self.location {
            Location::NodeTypeIndex(value) => {
                println!("{:?}", graph.edges(*value));
                println!("{:?}", route_forward_name);
                println!("{:?}", route_backward_name);
                let filtered_route_1 = graph.edges(*value).filter(|x|
                    x.weight().lock().unwrap().name == route_forward_name);
                let filtered_route_2 = graph.edges(*value).filter(|x|
                        x.weight().lock().unwrap().name == route_backward_name);
                let filtered_route_3 = graph.edges(*value).filter(|x|
                    x.weight().lock().unwrap().name == route_forward_name);
                for route in filtered_route_1 {
                    let unwrapped_route = route.weight().lock().unwrap().clone();
                    let suffix_forward = unwrapped_route.name.chars().last().unwrap() == 'f';
                    if self.dir_forward == suffix_forward {
                        self.location = Location::EdgeTypeIndex(route.id());
                        return
                    } else {
                        continue
                    }
                };
                self.dir_forward = false;
                for route in filtered_route_2 {
                    let unwrapped_route = route.weight().lock().unwrap().clone();
                    let suffix_forward = unwrapped_route.name.chars().last().unwrap() == 'f';
                    if self.dir_forward == suffix_forward {
                        self.location = Location::EdgeTypeIndex(route.id());
                        return
                    } else {
                        continue
                    }
                };
                self.dir_forward = true;
                for route in filtered_route_3 {
                    let unwrapped_route = route.weight().lock().unwrap().clone();
                    let suffix_forward = unwrapped_route.name.chars().last().unwrap() == 'f';
                    if self.dir_forward == suffix_forward {
                        self.location = Location::EdgeTypeIndex(route.id());
                        return
                    } else {
                        continue
                    }
                };
            },
            Location::EdgeTypeIndex(value) => {let filtered_edge = graph.edge_endpoints(*value).unwrap().1;
                self.location = Location::NodeTypeIndex(filtered_edge);
            }
            _ => panic!("TrainLocation is neither NodeIndex nor EdgeIndex")
        };

    }

}

impl TrainRegister {
    pub fn new(register_name: String) -> TrainRegister {
        TrainRegister {
            name: register_name,
            next_train_id: 0 as u32,
            train_list: vec![]
        }
    }
    pub fn add_train(&mut self, train_type: TrainType, location: Location,
                     route_name: String, dir_forward: bool, model: String) -> u32 {
        let new_train = Train::new(&mut self.next_train_id, train_type, location,
                                   route_name, dir_forward, model);
        self.train_list.push(new_train);
        self.next_train_id.clone() - 1
    }
}

#[derive(Debug, PartialEq)]
pub enum Location {
    NodeTypeIndex(NodeIndex),
    EdgeTypeIndex(EdgeIndex)
}

#[cfg(test)]
mod tests {
    use petgraph::stable_graph::StableGraph;
    use super::*;
    use crate::graph;
    use crate::train::Location;
    use crate::train::Location::EdgeTypeIndex;

    fn construct_scenario(fake_id: &mut u32) -> (StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                                                 NodeIndex, NodeIndex, NodeIndex) {
        let mut test_graph =
            StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
        let station1 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Tokyo"), vec![(8, TrainType::LowSpeed),(2, TrainType::HighSpeed)]);
        let station2 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Nagoya"), vec![(3, TrainType::LowSpeed),(1, TrainType::HighSpeed)]);
        let station3 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Osaka"), vec![(5, TrainType::LowSpeed),(2, TrainType::HighSpeed)]);
        graph::add_route_to_graph(&mut test_graph, station1, station2,
                                  fake_id, String::from("Tokaido_Shinkansen"), true);
        graph::add_route_to_graph(&mut test_graph, station2, station3,
                                  fake_id, String::from("Tokaido_Shinkansen"), true);
        (test_graph, station1, station2, station3)
    }

    #[test]
    fn moving_train_from_node() {
        let mut fake_graph_id= 0;
        let (mut test_graph, station1,
            station2, station3) = construct_scenario(&mut fake_graph_id);
        let mut test_fleet = TrainRegister::new(String::from("Shinkansen_fleet"));
        test_fleet.add_train(TrainType::HighSpeed, Location::NodeTypeIndex(station1),
                             String::from("Tokaido_Shinkansen"), true,
                             String::from("Shinkansen"));
        println!("Starting at NodeIndex(0)");
        test_fleet.train_list[0].move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].location, EdgeTypeIndex(test_graph.find_edge(station1, station2).unwrap()));
        test_fleet.train_list[0].move_forward(&test_graph);
        test_fleet.train_list[0].move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].location, EdgeTypeIndex(test_graph.find_edge(station2, station3).unwrap()));
        test_fleet.train_list[0].move_forward(&test_graph);
        test_fleet.train_list[0].move_forward(&test_graph);
        println!("turnaround backwards worked!");
        test_fleet.train_list[0].move_forward(&test_graph);
        test_fleet.train_list[0].move_forward(&test_graph);
        test_fleet.train_list[0].move_forward(&test_graph);
        test_fleet.train_list[0].move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].location, EdgeTypeIndex(test_graph.find_edge(station1, station2).unwrap()));
        println!("turnaround forwards worked!");
    }
}