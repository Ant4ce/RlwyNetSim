pub mod station;
pub mod train;
pub mod line;

use std::collections::HashMap;
use crate::train::TrainType;

fn main() {
    
    let mut train_id_counter: u16 = 0; 
    let mut station_id_counter: u32 = 0; 
    let mut route_id_counter: u32 = 0; 


    //let mut station_identification: HashMap<u32, station::Station> = HashMap::new();

    // when calling the constructor of station be sure to 
    // insert the returned station Object into the HashMap.

    //let test_station = station::Station::new(&mut station_id_counter, "ASTAT".to_string(), station::Station::platform_gen(2, TrainType::LowSpeed) ); 

    //let test_dos = station::Station::new(&mut station_id_counter, "dos".to_string(), station::Station::platform_gen(4, TrainType::LowSpeed));

    //println!("{:?}", test_station);
    //println!("{:?}", test_dos);
    let mut test_station = station::Station::new(&mut station_id_counter, "AStat".to_string(), vec![(TrainType::LowSpeed, 3), (TrainType::Freight, 5)]);
    //println!("{:?}", test_station);

    //println!("{}", test_station.available_platform(TrainType::Freight).unwrap());

    test_station.enter_station(test_station.available_platform(TrainType::Freight).unwrap());
    println!("{:?}", test_station.enter_station(5));
    println!("{:?}", test_station);
}


