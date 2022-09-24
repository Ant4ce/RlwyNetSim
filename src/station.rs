//use self::platform;
mod platform; 

pub struct Station {
    platforms: Vec<platform::Platform>,
    id: u16,
    name: String,
}

use crate::train::TrainType;

impl Station {
    fn new(id: &mut u16, name: String, platforms: Vec<platform::Platform> ) -> Station {

        *id += 1;


        Station {
            platforms: platforms,
            id: id.clone(),
            name: name,
        }
    }
    fn platform_gen(number: u8,station_id: u16 ,platform_type: TrainType) -> Vec<platform::Platform> {
        

        let mut plat_vec = Vec::new();

        for i in 0..number {
            
            let a_plat = platform::Platform::new(i, station_id, platform_type);
            plat_vec.push(a_plat);
        } 
        plat_vec
    }
}
