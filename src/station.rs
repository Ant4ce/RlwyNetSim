mod platform; 


use crate::train::TrainType;

#[derive(Debug)]
pub struct Station {
    platforms: Vec<platform::Platform>,
    id: u32,
    name: String,
}

pub enum PlatformError {
        Booking,
}


impl Station {
    pub fn new(id: &mut u32, name: String, platform_nums: Vec<(TrainType, u8)> /*platforms: Vec<platform::Platform>*/ ) -> Station{
        
        let mut platform_id : u8 = 0;

        *id += 1;

        let mut all_platforms : Vec<platform::Platform> = Vec::new();
        for (pf_type, pf_num) in platform_nums {
            for plat in Self::platform_gen(pf_num, pf_type, &mut platform_id) {
                let copy = plat.clone();
                all_platforms.push(plat);
                println!("pushed a platform: {:?}", copy );
            }
        }

        Station {
            platforms: all_platforms,
            id: id.clone(),
            name: name,
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

    // method to get an empty/available platform. This is useful to 
    // allocate a spot for a new train to be spawned in.
    pub fn available_platform(&self, plat_type: TrainType) -> Option<u8> {
        
        for plat in &self.platforms {
            match plat.occupied {
                false => match plat.platform_type {
                    plat_type => Some(plat.id.clone()),
                    _ => continue,
                },
                _ => continue,
            };
        }
        None
    }

    pub fn enter_station( &mut self, booking_id: u8) -> Result<(), PlatformError> {
        for mut plat in &mut self.platforms {
            match plat.id {
                booking_id => {plat.occupied = true; ()}
                _ => continue
            }
        }
        Err(PlatformError::Booking)
    }
}
