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
    pub fn new(id: &mut u32, name: String, platforms: Vec<platform::Platform> ) -> Station{

        *id += 1;


        Station {
            platforms: platforms,
            id: id.clone(),
            name: name,
        }
    }

    pub fn platform_gen(number: u8, platform_type: TrainType) -> Vec<platform::Platform> {
        let mut plat_vec = Vec::new();

        for i in 0..number {
            let a_plat = platform::Platform::new(i, platform_type);
            plat_vec.push(a_plat);
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
