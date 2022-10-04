
pub struct Route {
    id: u32,
    name: String,
    start_station: u16,
    end_station: u16,
}

impl Route {
    fn new(id: &mut u32, name: String, start_station: u16, end_station: u16) -> Route {

        *id += 1;
        
        Route{
            id: id.clone(),
            name,
            start_station,
            end_station,
        }
    }
}
