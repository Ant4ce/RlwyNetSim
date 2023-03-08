use std::fmt::{Display, Formatter};
use crate::train::TrainType;

/// Used by Station
///
/// In fact, all interactions with Platform should be done via Station and it's wrappers
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
            platform_type: platform_type,
        }
    }
}

const OCCUPIED: &str = "occupied";
const FREE: &str = "free";
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let is_occupied;
        if self.occupied {
            is_occupied = String::from(OCCUPIED)
        }
        else {
            is_occupied = String::from(FREE);
        }
        write!(f, "{:?}, {:?}, {:?}", self.id, self.platform_type, is_occupied)
    }

}