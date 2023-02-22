use std::collections::HashMap;
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use crate::station::Station;
use crate::route::Route;

#[derive(Debug, PartialEq)]
pub struct Train<T> {
    id: u32,
    train_type: TrainType,
    location: Location<T>,
    route_name: String,
    dir_forward: bool,
    model: String,
}

pub struct TrainRegister<T> {
    name: String,
    next_train_id: u32,
    train_list: Vec<Train<T>>
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TrainType {
    LowSpeed,
    Freight,
    HighSpeed,
}

impl<T> Train<T> {
    fn new(id: &mut u32, train_type: TrainType, location: Location<T>,
               route_name: String, dir_forward: bool, model: String) -> Train<T> {

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

}

impl<T> TrainRegister<T> {
    pub fn new(register_name: String) -> TrainRegister<T> {
        TrainRegister {
            name: register_name,
            next_train_id: 0 as u32,
            train_list: vec![]
        }
    }
    pub fn add_train(&mut self, train_type: TrainType, location: Location<T>,
                     route_name: String, dir_forward: bool, model: String) -> u32 {
        let new_train = Train::new(&mut self.next_train_id, train_type, location,
                                   route_name, dir_forward, model);
        self.train_list.push(new_train);
        self.next_train_id.clone() - 1
    }
}

#[derive(Debug, PartialEq)]
pub struct Location<T> {
    index: T,
}

impl<T> Location<T> {
    pub fn get_location(&self) -> &T{
        &self.index
    }
}