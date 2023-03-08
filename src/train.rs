use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
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

/// A Register to contain many trains.
///
/// With such collection, all trains in a simulation can be moved according to the event loop.
/// It also contains the id used for generating trains.
#[derive(Debug)]
pub struct TrainRegister {
    name: String,
    next_train_id: u32,
    pub train_list: Vec<Arc<Mutex<Train>>>,
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
            model,
        };
        *id += 1;
        new_train
    }
    /// Method for train to move forward.
    ///
    /// The current location is defined by the enum Train::Location, that tells whether the current
    /// index on the graph is a NodeIndex or EdgeIndex.
    ///
    /// When on an edge, it moves to the connected Node in the edges direction,
    /// as a directed graph is used.
    ///
    /// When train is currently on a node, find the next edge to go on and change location with the
    /// helper function [change_train_location](Train::change_train_location).
    ///
    pub fn move_forward(&mut self, graph: &Arc<RwLock<petgraph::stable_graph::StableGraph
    <Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>) {

        let (mut route_forward_name, mut route_backward_name) =
            (self.route_name.clone(), self.route_name.clone());
        route_forward_name.push('f');
        route_backward_name.push('b');
        match self.location {
            Location::NodeTypeIndex(value) => {
                self.change_train_location(&*graph.read().unwrap(), &value,
                                     &route_forward_name, &route_backward_name, false)
            },
            Location::EdgeTypeIndex(value) => {
                let filtered_edge =
                    graph.read().unwrap().edge_endpoints(value).unwrap().1;
                self.location = Location::NodeTypeIndex(filtered_edge);
            }
        };
    }
    /// Function to find, and switch Location to, the next appropriate Edge from a Node.
    ///
    /// Iterates over all outgoing edges of a node, and filters the edges that match line with the
    /// Trains line. Differentiate forward- and backward direction of the line by the last letter and
    /// match that with the trains own direction.
    ///
    /// If no edge exists for the direction that the train is facing, on the line the train is
    /// assigned to, the trains direction is flipped and the function is called recursively.
    ///
    /// It can call itself once recursively and panics if doing so a second time, because then there
    /// seems to be no path the train can take.
    ///
    // TODO: replace panic! on second recursive call with error handling
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
                    self.location = Location::EdgeTypeIndex(y.id());
                } else if self.dir_forward == false &&
                    &y.weight().lock().unwrap().name.chars().last().unwrap() == &'b' {
                    self.location = Location::EdgeTypeIndex(y.id());
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
/// Compares forward- and backward variations of route names in Train and compares them to the
/// route name in the graph.
///
/// Note: The name of a route in a graph will either end with "f" or "b".
/// "F" meaning forwards and "b" meaning backwards.
///
/// # Examples
/// ```
/// let (name_in_graph, route_name_fo, route_name_ba) =
///     String::from("BerlinPotsdamf"),
///     String::from("BerlinPotsdamf"),
///     String::from("BerlinPotsdamb");
/// asserteq!(true, compare_route_names(name_in_graph, route_name_fo, route_name_ba))
/// ```
fn compare_route_names(name_in_graph: &String,
                       route_name_forward_direction: &String,
                       route_name_back_direction: &String) -> bool{
    if name_in_graph == route_name_forward_direction ||
        name_in_graph == route_name_back_direction { true }
    else { false }
}
impl Display for TrainType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
    /// This is the interface to use when creating a new train, as Train::new() is private
    pub fn add_train(&mut self, train_type: TrainType, location: Location,
                     route_name: String, dir_forward: bool, model: String) -> u32 {
        let new_train = Train::new(&mut self.next_train_id, train_type, location,
                                   route_name, dir_forward, model);
        self.train_list.push(Arc::new(Mutex::new(new_train)));
        self.next_train_id.clone() - 1
    }
}

/// Used solely for the train to take appropriate method for moving forward.
#[derive(Debug, PartialEq)]
pub enum Location {
    NodeTypeIndex(NodeIndex),
    EdgeTypeIndex(EdgeIndex)
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;
    use petgraph::stable_graph::StableGraph;
    use super::*;
    use crate::graph;
    use crate::train::Location::{EdgeTypeIndex, NodeTypeIndex};

    fn construct_scenario(fake_id: &mut u32) -> (Arc<RwLock<StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>,
                                                 NodeIndex, NodeIndex, NodeIndex) {
        let mut test_graph =
            Arc::new(RwLock::new(StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new()));
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
        let (test_graph, station1,
            station2, station3) = construct_scenario(&mut fake_graph_id);
        let mut test_fleet = TrainRegister::new(String::from("Shinkansen_fleet"));
        test_fleet.add_train(TrainType::HighSpeed, Location::NodeTypeIndex(station1),
                             String::from("Tokaido_Shinkansen"), true,
                             String::from("Shinkansen"));
        println!("Starting at NodeIndex(0)");
        test_fleet.train_list[0].lock().unwrap().move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].lock().unwrap().location,
                   EdgeTypeIndex(test_graph.read().unwrap().find_edge(station1, station2).unwrap()));
        for _ in 0..3 { test_fleet.train_list[0].lock().unwrap().move_forward(&test_graph); }
        assert_eq!(test_fleet.train_list[0].lock().unwrap().location,
                   NodeTypeIndex(station3));
        println!("successfully reached endstation, time for a turnaround!");
        test_fleet.train_list[0].lock().unwrap().move_forward(&test_graph);
        assert_eq!(test_fleet.train_list[0].lock().unwrap().location,
                   EdgeTypeIndex(test_graph.read().unwrap().find_edge(station3, station2).unwrap()));
        println!("turnaround backwards worked!");
        for _ in 1..5 { test_fleet.train_list[0].lock().unwrap().move_forward(&test_graph); }
        assert_eq!(test_fleet.train_list[0].lock().unwrap().location,
                   EdgeTypeIndex(test_graph.read().unwrap().find_edge(station1, station2).unwrap()));
        println!("turnaround forwards worked, the train go around infinitely");
    }
}