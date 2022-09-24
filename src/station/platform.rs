use crate::train::TrainType;

pub struct Platform {
    id: u8,
    station_id: u16, 
    occupied: bool,
    platform_type: TrainType,
}

impl Platform {
    pub fn new(id: u8, station_id: u16 , platform_type: TrainType) -> Platform {
        
        
        Platform {
            id,
            station_id,
            occupied: false,
            platform_type: TrainType::LowSpeed,
        }
    }
}
