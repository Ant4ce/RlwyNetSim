#[derive(PartialEq, Clone, Debug)]
pub struct Route {
    pub id: u32,
    pub name: String,
    pub start_station: u32,
    pub end_station: u32,
}

impl Route {
    pub fn new(
        id: &mut u32,
        name: String,
        start_station: u32,
        end_station: u32) -> Route {

        *id += 1;
        
        Route{
            id: id.clone(),
            name,
            start_station,
            end_station,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_route() {
        let test_route = Route::new(&mut 0, String::from("Geneva"), 2, 3);
        assert_eq!((test_route.id, test_route.name, test_route.start_station, test_route.end_station),
                   (1, String::from("Geneva"), 2, 3));
    }
}