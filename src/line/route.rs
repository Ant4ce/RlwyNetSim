#[derive(PartialEq)]
pub struct Route<'a> {
    pub id: u32,
    pub name: &'a String,
    pub start_station: u32,
    pub end_station: u32,
}

impl<'a> Route<'a> {
    pub fn new(id: &mut u32, name: &'a String, start_station: u32, end_station: u32) -> Route<'a> {

        *id += 1;
        
        Route{
            id: id.clone(),
            name,
            start_station,
            end_station,
        }
    }
}
