#[derive(PartialEq, Clone, Debug)]
pub struct Route {
    pub id: u32,
    pub name: String,
}

impl Route {
    pub fn new(id: &mut u32, name: String) -> Route {

        let route = Route{
            id: id.clone(),
            name,
        };
        *id += 1;
        route
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_route() {
        let test_route = Route::new(&mut 0, String::from("Geneva"));
        assert_eq!((test_route.id, test_route.name),
                   (0, String::from("Geneva")));
    }
}