#[derive(PartialEq, Clone, Debug)]
pub struct Edger {
    id: u32,
    name: String,
    start_station: u32,
    end_station: u32,
}

impl Edger {
    pub fn new(id: &mut u32, name: String, start_station: u32, end_station: u32) -> Edger {

        let UnitEdge = Edger{
           id: id.clone(),
           name,
           start_station,
           end_station,
        };
        *id += 1;
        UnitEdge
    }
}
