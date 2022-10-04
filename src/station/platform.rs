use crate::train::TrainType;

pub struct Platform {
    pub id: u8,
    pub occupied: bool,
    pub platform_type: TrainType,
}

impl Platform {
    pub fn new(id: u8, platform_type: TrainType) -> Platform {
        
        
        Platform {
            id,
            occupied: false,
            platform_type: TrainType::LowSpeed,
        }
    }
}
