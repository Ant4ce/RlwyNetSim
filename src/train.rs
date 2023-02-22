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
    fn move_forward(&mut self, graph: &petgraph::stable_graph::StableGraph
    <Arc<Mutex<Station>>, Arc<Mutex<Route>>>) {
        match &self.location {
            Location::NodeTypeIndex(value) => {let filtered_routes = graph.edges(*value).filter(|x|
                x.weight().lock().unwrap().name == self.route_name);
                for route in filtered_routes {
                    let unwrapped_route = route.weight().lock().unwrap().clone();
                    let suffix_forward = unwrapped_route.name.chars().last().unwrap() == 'f';
                    if self.dir_forward == suffix_forward {
                        self.location = Location::EdgeTypeIndex(route.id());
                    } else {
                        continue
                    }
                }
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
    use petgraph::prelude::StableGraph;
    use super::*;
    use crate::graph;

    fn construct_scenario(fake_id: &mut u32) -> StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>> {
        let mut test_graph =
            StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
        let station1 = test_graph.add_station_to_graph(&mut test_graph, fake_id,
                                       String::from("Berlin"), vec![(1, TrainType::LowSpeed)]);
    }

    #[test]
    fn moving_train_from_node() {

    }
}