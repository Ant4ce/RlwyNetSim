
struct Route<'a> {
    id: u32,
    name: &'a String,
    start_station: u32,
    end_station: u32,
}

impl<'a> Route<'a> {
    fn new(id: &'a mut u32, name: &'a String, start_station: u32, end_station: u32) -> Route<'a> {

        *id += 1;
        
        Route{
            id: id.clone(),
            name,
            start_station,
            end_station,
        }
    }
}
