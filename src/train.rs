use std::collections::HashMap;
use crate::station::Station;

struct Train {
    id: u32,
    model: String,
    dir_forward: bool,
    train_type: TrainType, 
    location: u32,
    route: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TrainType {
    LowSpeed,
    Freight,
    HighSpeed,
}

impl Train {
    fn new(id: &mut u32 , model: String, dir_forward: bool, train_type: TrainType, location: u32, route: String, station_identifier: &mut HashMap<u32, Station>) -> Train {
        *id  += 1;

        let start_station: &mut Station = station_identifier.get_mut(&location).unwrap();
        // TODO handle lifetimes for the locations
        let empty_plat = start_station.available_platform(TrainType::LowSpeed).unwrap();
        start_station.enter_station(empty_plat);

        Train {
            id: id.clone(),
            model: "Passenger".to_string(),
            dir_forward: true,
            train_type: TrainType::LowSpeed,
            location: location,
            route: route,
        }

    }
    
    // This method will in the future be used to get station ID 
    // based on which station was clicked on, but for now just gets the
    // location (so ID of a station we pass it) and returns it. 
    fn spawn_loc(location: u32) -> u32 {
        location
    } 
}
