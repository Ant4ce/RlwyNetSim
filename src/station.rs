mod platform; 


use crate::train::TrainType;

#[derive( Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Station {
    id: u32,
    name: String,
    platforms: Vec<platform::Platform>,
}

#[derive(Debug)]
pub enum PlatformError {
        Booking,
}


impl Station {
    pub fn new(id: &mut u32, name: String, platform_nums: Vec<(TrainType, u8)> ) -> Station{
        
        let mut platform_id : u8 = 0;

        *id += 1;

        let mut all_platforms : Vec<platform::Platform> = Vec::new();
        for (pf_type, pf_num) in platform_nums {
            for plat in Self::platform_gen(pf_num, pf_type, &mut platform_id) {
                //let copy = plat.clone();
                all_platforms.push(plat);
                //println!("pushed a platform: {:?}", copy );
            }
        }

        Station {
            id: id.clone(),
            name: name,
            platforms: all_platforms,
        }
    }

    pub fn platform_gen(number: u8, platform_type: TrainType, id: &mut u8) -> Vec<platform::Platform> {
        let mut plat_vec = Vec::new();

        for _ in 0..number {
            let a_plat = platform::Platform::new(id.clone(), platform_type);
            plat_vec.push(a_plat);
            *id += 1;
        } 
        plat_vec
    }

    // Hey this is cute. 
    // method to get an empty/available platform. This is useful to 
    // allocate a spot for a new train to be spawned in.
    pub fn available_platform(&self, plat_type: TrainType) -> Option<u8> {
        
        for plat in &self.platforms {
            if (plat.occupied == false && plat.platform_type == plat_type) {
                return Some(plat.id.clone())
            }
        }
        None
    }

    pub fn enter_station( &mut self, booking_id: u8) /*-> Result<(), PlatformError>*/ {
        // there is no need to loop over this, why have linear runtime ?? When we have the id to
        // access it straight away. UPDATE THIS. 
        //for mut plat in &mut self.platforms {
        //   // REMEMBER: match is PATTERN MATCHING, so it matches on patterns not specific values.
        //   // Here i use match guard, which is the "num if num == booking_id". This allows me to
        //   // match on a specific value. Just matching on booking ID doesn't work.    
        //    match plat.id {
        //        num if num == booking_id  => {plat.occupied = true;  return Ok(())},
        //        _ => continue,
        //    }
        //}
        //Err(PlatformError::Booking)
        
        //*Improved version*
        // Indexing into array requires usize so converted the u8 from booking_id to usize. 
        self.platforms[usize::from(booking_id)].occupied = true; 
        //Ok(())
        //Err(PlatformError::Booking)
    }

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
        
        let unit_station =  Station::new(&mut 0, "TesterBoi".to_string(), vec![(TrainType::Freight, 3), (TrainType::LowSpeed, 1)]);

        assert_eq!(unit_station.name, "TesterBoi".to_string());
        assert_eq!(unit_station.platforms[0], platform::Platform { id: 0, occupied: false, platform_type: TrainType::Freight});
        assert_eq!(unit_station.platforms[3], platform::Platform { id: 3, occupied: false, platform_type: TrainType::LowSpeed});
    }

    #[test]
    fn plat_availability_test() {

        let unit_station =  Station::new(&mut 0, "TesterBoi".to_string(), vec![(TrainType::HighSpeed, 3), (TrainType::LowSpeed, 2)]);
        
        assert_eq!(unit_station.available_platform(TrainType::LowSpeed).unwrap(), 3);

    }
    #[test]
    fn occupancy() {
        let mut unit_station = Station::new(&mut 0, "TesterBoi2".to_string(), vec![(TrainType::Freight, 5), (TrainType::LowSpeed, 2)]);
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

