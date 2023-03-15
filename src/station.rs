mod platform;

use std::fmt::{Display, Formatter};
use crate::train::TrainType;

/// Station Struct
/// These are the nodes in our graph.
#[derive( Debug, PartialEq)]
pub struct Station {
    pub id: u32,
    pub name: String,
    pub platforms: Vec<platform::Platform>,
}
/// Used to specify the different kinds of errors that the station constructor might throw.
#[derive(Debug)]
pub enum PlatformError {
        Booking,
}

impl Display for Station {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}, with {:?} Platforms", self.id, self.name, self.platforms.len())
    }
}

impl Station {
    /// Creates new Station
    ///
    /// Requires a name (String) and a Vector of platforms holding tuples of the number
    /// of platforms you want for each TrainType.
    ///
    /// # Examples:
    /// ```
    /// // This creates a station called "Berlin" with 1 platform of type Freight
    /// // and 3 of type LowSpeed.
    /// let my_station = Station::new(id: &mut u32, name: String::from("Berlin"),
    ///          platforms: vec![(1, TrainType::Freight), (3, TrainType::LowSpeed)]);
    /// ```
    pub fn new(id: &mut u32, name: String, platform_nums: Vec<(u8, TrainType)> ) -> Station{
        
        let mut platform_id : u8 = 0;

        let mut all_platforms : Vec<platform::Platform> = Vec::new();
        for (pf_num, pf_type) in platform_nums {
            for plat in Self::platform_gen(pf_num, pf_type, &mut platform_id) {
                all_platforms.push(plat);
            }
        }

        let the_station = Station {
            id: id.clone(),
            name: name,
            platforms: all_platforms,
        };

        *id += 1;

        the_station
    }

    /// Creates platforms
    ///
    /// Used to create a Vec with the given number of platforms for the given type.
    /// Note the last number is the id (u8) and is handled in other
    /// parts of Station constructor.
    ///
    /// # Example
    /// ```
    /// // This creates 3 platforms of type TrainType::LowSpeed.
    /// let my_platforms = Station::platform_gen(3 , TrainType::LowSpeed, 1);
    /// ```
    // TODO: make private
    pub fn platform_gen(number: u8, platform_type: TrainType, id: &mut u8) -> Vec<platform::Platform> {
        let mut plat_vec = Vec::new();

        for _ in 0..number {
            let a_plat = platform::Platform::new(id.clone(), platform_type);
            plat_vec.push(a_plat);
            *id += 1;
        } 
        plat_vec
    }

    /// Returns id of an unoccupied platform of the specified TrainType.
    /// The return type is Option<u8> where the u8 would be the id.
    ///
    /// # Example
    /// ```
    /// // Gives the id (wrapped in Option) of a station of type HighSpeed.
    /// let my_open_platform = Station::available_platform( TrainType::HighSpeed);
    /// ```
    pub fn available_platform(&self, plat_type: TrainType) -> Option<u8> {
        
        for plat in &self.platforms {
            if plat.occupied == false && plat.platform_type == plat_type {
                return Some(plat.id.clone())
            }
        }
        None
    }
    /// Change a platforms occupied status to true.
    ///
    /// # Example
    /// ```
    /// // changes the occupied status of platform with id = 4 to true.
    /// Station::enter_station(4);
    /// ```
    pub fn enter_station( &mut self, booking_id: u8) /*-> Result<(), PlatformError>*/ {
        // TODO: Add error handling for when this fails, consider returning a Result<> type
        //  with error type PlatformError.
        self.platforms[usize::from(booking_id)].occupied = true;
    }

    /// Change a platforms occupied status to false.
    ///
    /// # Example
    /// ```
    /// // changes the occupied status of platform with id = 2 to false.
    /// Station::leave_station(2);
    /// ```
    pub fn leave_station( &mut self, plat_id: u8) {

        self.platforms[usize::from(plat_id)].occupied = false;
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //Tests run on the generation of platforms.
    #[test]
    fn num_of_plats_gen() {
        assert_eq!(Station::platform_gen(6, TrainType::Freight, &mut 0).len(), 6);
    }


    //Tests run on the generation of stations.
    //Created test station to see if values are what we expect.
    #[test]
    fn station_attributes() {
        
        let unit_station =  Station::new(&mut 0, "TesterBoi".to_string(), vec![( 3,TrainType::Freight), ( 1, TrainType::LowSpeed,)]);

        assert_eq!(unit_station.name, "TesterBoi".to_string());
        assert_eq!(unit_station.platforms[0], platform::Platform { id: 0, occupied: false, platform_type: TrainType::Freight});
        assert_eq!(unit_station.platforms[3], platform::Platform { id: 3, occupied: false, platform_type: TrainType::LowSpeed});
    }

    #[test]
    fn plat_availability_test() {

        let unit_station =  Station::new(&mut 0, "TesterBoi".to_string(), vec![(3, TrainType::HighSpeed), (2, TrainType::LowSpeed)]);
        
        assert_eq!(unit_station.available_platform(TrainType::LowSpeed).unwrap(), 3);

    }
    #[test]
    fn occupancy() {
        let mut unit_station = Station::new(&mut 0, "TesterBoi2".to_string(), vec![(5, TrainType::Freight), (2, TrainType::LowSpeed)]);
        unit_station.enter_station(unit_station.available_platform(TrainType::LowSpeed).unwrap());

        assert_eq!(unit_station.platforms[5].occupied, true);
        assert_eq!(unit_station.platforms[4].occupied, false);
        assert_eq!(unit_station.platforms[6].occupied, false);

        unit_station.enter_station(unit_station.available_platform(TrainType::LowSpeed).unwrap());

        assert_eq!(unit_station.platforms[6].occupied, true);
        assert_eq!(unit_station.platforms[5].occupied, true);
        assert_eq!(unit_station.platforms[4].occupied, false);

        unit_station.leave_station(5);
        assert_eq!(unit_station.platforms[5].occupied, false);
        assert_eq!(unit_station.platforms[6].occupied, true);
        assert_eq!(unit_station.platforms[4].occupied, false);
    }
}

