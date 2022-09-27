//use self::platform

mod platform; 


use std::collections::HashMap;
use crate::train::TrainType;

pub let mut station_identification = HashMap::new();

pub struct Station {
    platforms: Vec<platform::Platform>,
    id: u32,
    name: String,
}


impl Station {
    fn new(id: &mut u32, name: String, platforms: Vec<platform::Platform> ) -> u32 {

        *id += 1;


        Station {
            platforms: platforms,
            id: id.clone(),
            name: name,
        };
        // adding the newly created station and it's respective ID to 
        // the HashMap for identification.
        station_identification.insert(id.clone(), Station);
        id.clone()
    }
    fn platform_gen(number: u8,station_id: u32 ,platform_type: TrainType) -> Vec<platform::Platform> {
        

        let mut plat_vec = Vec::new();

        for i in 0..number {
            
            let a_plat = platform::Platform::new(i, station_id, platform_type);
            plat_vec.push(a_plat);
        } 
        plat_vec
    }

    // method to get an empty/available platform. This is useful to 
    // allocate a spot for a new train to be spawned in.
    fn available_platform(self, plat_type: TrainType) -> Option<u8> {
        
        for plat in self.platforms {
            match plat.occupied {
                false => match plat.platform_type {
                    plat_type => Some(plat.id),
                    _ => continue,
                },
                _ => continue,
            }
        }
        None
    }

    enum PlatformError {
        Booking,
    }


    fn enter_station(self, booking_id: u8) -> Result<(), PlatformError> {
        for plat in self.platform {
            match plat.id {
                booking_id => {plat.occupation = true; Ok()}
                _ => continue
            }
        }
        Err(PlatformError::Booking)
    }
}
