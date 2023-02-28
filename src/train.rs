use std::sync::{Arc, mpsc, Mutex};
use std::thread::JoinHandle;
use petgraph::stable_graph::{NodeIndex, EdgeIndex, EdgeReference, StableGraph};
use petgraph::visit::EdgeRef;
use crate::station::Station;
use crate::route::Route;
use crate::threadpool::*;

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
    pub train_list: Vec<Train>
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
        let (mut route_forward_name, mut route_backward_name) =
            (self.route_name.clone(), self.route_name.clone());
        route_forward_name.push('f');
        route_backward_name.push('b');
        match self.location {
            Location::NodeTypeIndex(value) => {
                self.change_train_location(graph, &value,
                                     &route_forward_name, &route_backward_name, false)
            },
            Location::EdgeTypeIndex(value) => {
                let filtered_edge =
                    graph.edge_endpoints(value).unwrap().1;
                self.location = Location::NodeTypeIndex(filtered_edge);
            }
            _ => panic!("TrainLocation is neither NodeIndex nor EdgeIndex")
        };

    }
    fn change_train_location(&mut self,
                             current_graph: &petgraph::stable_graph::
                                StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                             current_location: &NodeIndex,
                             route_forward_name: &String,
                             route_backward_name: &String,
                             called_recurs: bool) {
        current_graph.edges(*current_location)
            //filter to get route's of the same name in either direction
            .filter(|y| compare_route_names(
                &y.weight().lock().unwrap().name,
                &route_forward_name, &route_backward_name))
            //now move forward or backward depending on train direction and available route
            .for_each(|y| {
                if self.dir_forward &&
                    &y.weight().lock().unwrap().name.chars().last().unwrap() == &'f' {
                    self.location = Location::EdgeTypeIndex((y.id()));
                } else if self.dir_forward == false &&
                    &y.weight().lock().unwrap().name.chars().last().unwrap() == &'b' {
                    self.location = Location::EdgeTypeIndex((y.id()));
                }
            });
        if self.location != Location::NodeTypeIndex(*current_location) { return }
        if self.dir_forward { self.dir_forward = false }
        else { self.dir_forward = true }
        if called_recurs { panic!("cannot move Train forward in either direction!") }
        self.change_train_location(current_graph, current_location,
                                   route_forward_name, route_backward_name, true)
    }

}
/// Compares forward- and backward variations of route names in Train and compares them to the route name in the graph
///
/// # Examples
/// ```
/// let (name_in_graph, route_name_fo, route_name_ba) =
///     String::from("BerlinPotsdamf"),
///     String::from("BerlinPotsdamf"),
///     String::from("BerlinPotsdamb");
/// asserteq!(true, compare_route_names(name_in_graph, route_name_fo, route_name_ba)
/// ```
fn compare_route_names(name_in_graph: &String,
                       route_name_forward_direction: &String,
                       route_name_back_direction: &String) -> bool{
    if name_in_graph == route_name_forward_direction ||
        name_in_graph == route_name_back_direction { true }
    else { false }
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
    use crate::train::Location::{EdgeTypeIndex, NodeTypeIndex};

    fn construct_scenario(fake_id: &mut u32) -> (StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>,
                                                 NodeIndex, NodeIndex, NodeIndex) {
        let mut test_graph =
            StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
        let station1 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Tokyo"), vec![(8, TrainType::LowSpeed),
                                                       (2, TrainType::HighSpeed)]);
        let station2 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Nagoya"), vec![(3, TrainType::LowSpeed),
                                                        (1, TrainType::HighSpeed)]);
        let station3 = graph::add_station_to_graph(&mut test_graph, fake_id,
                           String::from("Osaka"), vec![(5, TrainType::LowSpeed),
                                                       (2, TrainType::HighSpeed)]);
        graph::add_route_to_graph(&mut test_graph, station1, station2, fake_id,
                                  String::from("Tokaido_Shinkansen"), true);
        graph::add_route_to_graph(&mut test_graph, station2, station3, fake_id,
                                  String::from("Tokaido_Shinkansen"), true);
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
        assert_eq!(test_fleet.train_list[0].location,
                   EdgeTypeIndex(test_graph.find_edge(station1, station2).unwrap()));
        for _ in 0..3 { test_fleet.train_list[0].move_forward(&test_graph); }
        assert_eq!(test_fleet.train_list[0].location,
                   NodeTypeIndex(station3));
        println!("successfully reached endstation, time for a turnaround!");
        test_fleet.train_list[0].move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].location,
                   EdgeTypeIndex(test_graph.find_edge(station3, station2).unwrap()));
        println!("turnaround backwards worked!");
        for _ in 1..5 { test_fleet.train_list[0].move_forward(&test_graph); }
        assert_eq!(test_fleet.train_list[0].location,
                   EdgeTypeIndex(test_graph.find_edge(station1, station2).unwrap()));
        println!("turnaround forwards worked, the train go around infinitely");
    }
}