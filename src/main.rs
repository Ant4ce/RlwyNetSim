pub mod station;
pub mod train;
pub mod route;

use std::collections::HashMap;
use crate::train::TrainType;

fn main() {
    
    let mut train_id_counter: u16 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 


    let mut station_identification: HashMap<u32, station::Station> = HashMap::new();
    // when calling the constructor of station be sure to 
    // insert the returned station Object into the HashMap.

    let test_station = station::Station::new(&mut station_id_counter, "ASTAT".to_string(), station::Station::platform_gen(2, TrainType::LowSpeed) ); 
}


