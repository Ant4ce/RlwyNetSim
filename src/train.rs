
struct Train {
    id: u16,
    model: String,
    dir_forward: bool,
    train_type: TrainType 
}

#[derive(Copy, Clone)]
pub enum TrainType {
    LowSpeed,
    Freight,
    HighSpeed,
}

impl Train {
    fn new(id: &mut u16 , model: String, dir_forward: bool, train_type: TrainType) -> Train {
        *id  += 1;

        Train {
            id: id.clone(),
            model: "Passenger".to_string(),
            dir_forward: true,
            train_type: TrainType::LowSpeed,
        }

    }
}
