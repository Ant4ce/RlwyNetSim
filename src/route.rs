use crate::station::Station;

struct Route<'a> {
    id: u32,
    name: String,
    start_station: u32,
    end_station: u32,
}

impl Route {
    fn new<&'a>(id: &mut u32, &'a name: String, start_station: u32, end_station: u32) -> &'a Route {

        *id += 1;
        
        Route{
            id: id.clone(),
            name,
            start_station,
            end_station,
        }
    }
}
