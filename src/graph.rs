use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::StableGraph;
use petgraph::stable_graph::{NodeIndex, EdgeIndex};
use crate::{station::Station, route::Route};
use crate::train::TrainType;

use petgraph::{Incoming, Outgoing};
use petgraph::visit::{EdgeRef};

/// Enum to hold various GraphError we can return for graph constructor.
#[derive(Debug, PartialEq)]
pub enum GraphError {
    RemovingStation,
}
pub struct Graph {
    pub graph: Arc<RwLock<StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>
}

impl Graph {
    pub fn new() -> Graph{
        Graph {
            graph: Arc::new(RwLock::new(StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new()))
        }
    }
    /// Adds a station (Node) to the graph and returns it's NodeIndex
    ///
    /// # Example
    /// ```
    /// let mut graph = StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
    /// let mut station_id_counter: u32 = 0;
    ///
    /// // Adds the station with name Warsaw to the graph. It has 3 LowSpeed and
    /// // 1 Freight type platform.
    /// let start_node = add_station_to_graph(&mut graph, &mut station_id_counter,
    ///     "Warsaw".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    /// ```
    pub fn add_station_to_graph(&mut self, id: &mut u32, name: String,
                                platforms : Vec<(u8, TrainType)>) -> NodeIndex {

        let new_station = Station::new(id, name, platforms);
        self.graph.write().unwrap().add_node(Arc::new(Mutex::new(new_station)))
    }
    /// Adds a route (Edge) to the graph, has no return.
    ///
    /// The function takes graph of type: &mut Arc<RwLock<StableGraph<Arc<Mutex<Station>>, Arc<Mutex<Route>>>>>
    /// The Arc's, RwLock's and Mutex's are all to make sure the graph works concurrently
    ///  for reading and writing access.
    ///  The other parameters are the two NodeIndex of the stations, the route id as u32,
    ///  the name of the Route as String and bideractional boolean to specify wether you want
    ///  a route going in both directions between the nodes.
    ///
    /// # Example
    /// ```
    /// let mut graph = StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
    /// let mut station_id_counter: u32 = 0;
    ///
    /// let start_node = add_station_to_graph(&mut graph, &mut station_id_counter,
    ///     "Warsaw".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    /// let end_node= add_station_to_graph(&mut graph, &mut station_id_counter,
    ///     "Eindhoven".to_string(), vec![(1, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);
    ///
    /// // This adds the route "IE1" between Warsaw and Eindhoven. The last parameter of true
    /// // means we want both a going route and a returning route.
    /// add_route_to_graph(&mut graph, start_node, end_node, &mut route_id_counter, String::from("IE1"), true);
    /// ```
//TODO: make sure that one station can not have two INCOMING routes of the same name without an
// outgoing route and the same for two OUTGOING routes without an incoming route.
    pub fn add_route_to_graph(&mut self,
                              station_a: NodeIndex, station_b: NodeIndex, route_id: &mut u32,
                              name: String, bidirectional: bool) -> ( Option<EdgeIndex>, Option<EdgeIndex> ) {
        let (mut name_f, mut name_b) = (name.clone(), name.clone());
        name_f.push('f');
        name_b.push('b');

        let new_route = Route::new(route_id, name_f);
        let new_route_reverse_direction = Route::new(route_id, name_b);

        let mut backward_edge = None;
        let forward_edge = Some(self.graph.write().unwrap().add_edge(station_a, station_b, Arc::new(Mutex::new(new_route))));
        if bidirectional == true {
            backward_edge = Some(self.graph.write().unwrap().add_edge(station_b, station_a, Arc::new(Mutex::new(new_route_reverse_direction))));
        }
        (forward_edge, backward_edge)
    }
    /// Removes specified station from graph
    ///
    /// This function will remove the specified node from the graph
    /// but not before creating new edges that will replace the edges that get
    /// removed along with the node. The edges it replaces are only those that
    /// have a incoming and outgoing one for their route. So if a line like "IE1"
    /// would pass through the node we are removing, and removing it would leave us with
    /// 2 disconnected "IE1" sections. This function makes sure that those 2 parts stay
    /// connected.
    ///
    /// # Example
    /// ```
    /// let mut graph = StableGraph::<Arc<Mutex<Station>>, Arc<Mutex<Route>>>::new();
    /// let mut station_id_counter: u32 = 0;
    /// let mut route_id_counter: u32 = 0;
    ///
    /// let start_node = add_station_to_graph(&mut graph, &mut station_id_counter,
    ///     "Warsaw".to_string(), vec![(3, TrainType::LowSpeed),(1, TrainType::Freight)]);
    ///
    /// remove_station_from_graph(&mut graph, start_node, &mut route_id_counter);
    /// ```
    pub fn remove_station_from_graph(&mut self, index_node: NodeIndex, id_route_counter : &mut u32) -> Result<(), GraphError>  {

        let graph_read = self.graph.read().unwrap();
        let e_neighbours_incoming = graph_read.edges_directed(index_node, Incoming);
        let e_neighbours_outgoing = graph_read.edges_directed(index_node, Outgoing);
        let mut edge_names_and_indexes: Vec<(String, EdgeIndex)> = vec![];

        for x in e_neighbours_incoming {
            let name_route = x.weight().lock().unwrap().name.clone();
            edge_names_and_indexes.push((name_route, x.id()));
        }
        for z in e_neighbours_outgoing {
            let name_route = z.weight().lock().unwrap().name.clone();
            edge_names_and_indexes.push((name_route, z.id()));
        }
        let mut related_routes: Vec<(String, Vec<EdgeIndex>)> = vec![];
        for (name, edge) in edge_names_and_indexes {
            let temp_item = related_routes.iter_mut().find(|x| *x.0 == name);
            match temp_item {
                Some(v)  => v.1.push(edge),
                None => related_routes.push((name, vec![edge])),
            };
        }
        //TODO: this currently filters out all related routes when there is not exactly 2 of them.
        // This is done because when there was only one route of its kind, it needs to be deleted, as it
        // visits no other station than the one deleted.
        // All other cases we do not yet handle.
        let pair_routes = related_routes.iter().filter(|x| x.1.len() == 2);
        drop(graph_read);
        for line in pair_routes {
            let route_name = line.0.clone();
            let first_edge = self.graph.read().unwrap().edge_endpoints(line.1[0]).unwrap();
            let second_edge = self.graph.read().unwrap().edge_endpoints(line.1[1]).unwrap();
            let new_edge: (NodeIndex, NodeIndex);
            if first_edge.0 == index_node {
                new_edge = (second_edge.0, first_edge.1);
            } else {
                new_edge = (first_edge.0, second_edge.1);
            }
            self.add_route_to_graph(new_edge.0, new_edge.1, id_route_counter, route_name, true);
        }

        let removed_node = self.graph.write().unwrap().remove_node(index_node);
        match removed_node {
            Some(_) => Ok(()),
            None => Err(GraphError::RemovingStation),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::stable_graph::StableGraph;
    use crate::train::TrainType;

    #[test]
    fn adding_station() {
        let mut test_graph = Graph::new();

        let test_graph_ind = test_graph.add_station_to_graph(&mut 0,
                                                  String::from("Berlin"),
                                                  vec![(1, TrainType::LowSpeed)]);
        assert_eq!(test_graph.graph.read().unwrap().node_weight(test_graph_ind).unwrap().lock().unwrap().name,
                   String::from("Berlin"));
        let compare_station = Station::new(&mut 0, String::from("Berlin"),
                         vec![(1, TrainType::LowSpeed)]);
        assert_eq!(*test_graph.graph.read().unwrap().node_weight(test_graph_ind).unwrap().lock().unwrap(), compare_station);
    }

    #[test]
    fn adding_route_as_edge() {
        let mut test_graph = Graph::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStreamf"));
        let compare_test_route_reverse = Route::new(&mut 3, String::from("NordStreamb"));

        let test_graph_ind_a = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        test_graph.add_route_to_graph(test_graph_ind_a,
                                      test_graph_ind_b, &mut fake_id,
                                      String::from("NordStream"), true);
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStreamf"));
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap(), compare_test_route);

        // testing that reverse direction edge was created.
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStreamb"));
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_b, test_graph_ind_a).unwrap())).unwrap().lock().unwrap(), compare_test_route_reverse);
    }
    #[test]
    fn adding_uni_directional_edge() {
        let mut test_graph = Graph::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStream2f"));

        let test_graph_ind_a = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        test_graph.add_route_to_graph(test_graph_ind_a,
                           test_graph_ind_b, &mut fake_id,
                           String::from("NordStream2"), false);
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap().name,
                   String::from("NordStream2f"));
        assert_eq!(*(test_graph.graph.read().unwrap().edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap())).unwrap().lock().unwrap(), compare_test_route);

    }
    #[test]
    #[should_panic]
    fn panicking_adding_uni_directional_edge() {
        let mut test_graph = Graph::new();

        let mut fake_id:u32 = 0;
        let compare_test_route = Route::new(&mut 2, String::from("NordStream2f"));
        Route::new(&mut 2, String::from("NordStream2b"));

        let test_graph_ind_a = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Berlin"),
                                                    vec![(1, TrainType::LowSpeed)]);
        let test_graph_ind_b = test_graph.add_station_to_graph(&mut fake_id,
                                                    String::from("Moscow"),
                                                    vec![(1, TrainType::LowSpeed),
                                                         (1, TrainType::HighSpeed)]);
        test_graph.add_route_to_graph(test_graph_ind_a,
                           test_graph_ind_b, &mut fake_id,
                           String::from("NordStream2"), false);
        let read_graph = test_graph.graph.read().unwrap();
        let forward_edge = read_graph.edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap());
        assert_eq!(*forward_edge.unwrap().lock().unwrap().name, String::from("NordStream2f"));
        assert_eq!(*forward_edge.unwrap().lock().unwrap(), compare_test_route);

        //check that reverse direction doesn't exist, this should cause a panic.
        let backward_edge = read_graph.edge_weight(test_graph.graph.read().unwrap().find_edge(test_graph_ind_a, test_graph_ind_b).unwrap());
        assert_eq!(*backward_edge.unwrap().lock().unwrap().name,String::from("NordStream2b"));
        assert_eq!(*backward_edge.unwrap().lock().unwrap(), compare_test_route);

    }
    #[test]
    fn removing_a_route_from_graph() {
        let mut test_graph = Graph::new();

        let mut fake_id: u32 = 0;
        let test_station_1 = test_graph.add_station_to_graph(&mut fake_id,
                                        String::from("Warsaw"),
                                        vec![(1, TrainType::Freight), (2, TrainType::LowSpeed)]);

        let test_station_2 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Madrid"),
                                                  vec![(1, TrainType::HighSpeed), (2, TrainType::LowSpeed)]);
        let test_station_3 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Paris"),
                                                  vec![(1, TrainType::Freight), (2, TrainType::HighSpeed)]);

        test_graph.add_route_to_graph(test_station_1, test_station_3, &mut fake_id, String::from("waris"), true);
        test_graph.add_route_to_graph(test_station_3, test_station_2, &mut fake_id, String::from("waris"), true);

        assert_eq!(true, test_graph.graph.read().unwrap().contains_node(test_station_3));

        let removal = test_graph.remove_station_from_graph(test_station_3, &mut fake_id);
        // add the assertequal, to test wether removal worked.

        assert_eq!(Ok(()), removal);
        assert_eq!(test_graph.graph.read().unwrap().edges_connecting(test_station_1, test_station_2).count(), 2);
        assert_eq!(false, test_graph.graph.read().unwrap().contains_node(test_station_3));

    }
    #[test]
    fn  test_major_station_removal() {

        let mut test_graph = Graph::new();


        let mut fake_id: u32 = 0;
        let test_station_1 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Warsaw"),
                                                  vec![(1, TrainType::Freight), (2, TrainType::LowSpeed)]);
        let test_station_2 = test_graph.add_station_to_graph( &mut fake_id,
                                                  String::from("Madrid"),
                                                  vec![(1, TrainType::HighSpeed), (2, TrainType::LowSpeed)]);
        let test_station_3 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Paris"),
                                                  vec![(1, TrainType::Freight), (3, TrainType::HighSpeed)]);
        let test_station_4 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("London"),
                                                  vec![(2, TrainType::Freight), (4, TrainType::HighSpeed)]);
        let test_station_5 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Milan"),
                                                  vec![(7, TrainType::Freight), (5, TrainType::HighSpeed)]);
        let test_station_6 = test_graph.add_station_to_graph(&mut fake_id,
                                                  String::from("Pontsarn"),
                                                  vec![(3, TrainType::LowSpeed), (2, TrainType::HighSpeed)]);

        test_graph.add_route_to_graph(test_station_1, test_station_3, &mut fake_id, String::from("waris"), false);
        test_graph.add_route_to_graph(test_station_3, test_station_2, &mut fake_id, String::from("waris"), false);
        test_graph.add_route_to_graph(test_station_4, test_station_3, &mut fake_id, String::from("Loilan"), false);
        test_graph.add_route_to_graph(test_station_3, test_station_5, &mut fake_id, String::from("Loilan"), false);
        test_graph.add_route_to_graph(test_station_3, test_station_6, &mut fake_id, String::from("Parn"), false);

        assert_eq!(true , test_graph.graph.read().unwrap().contains_node(test_station_3));

        let removal = test_graph.remove_station_from_graph(test_station_3, &mut fake_id);
        assert_eq!(Ok(()), removal);
        assert_eq!(test_graph.graph.read().unwrap().edges_connecting(test_station_1, test_station_2).count(), 1);
        assert_eq!(test_graph.graph.read().unwrap().edges_connecting(test_station_4, test_station_5).count(), 1);
        assert_eq!(test_graph.graph.read().unwrap().edges(test_station_6).count(), 0);
        assert_eq!(false, test_graph.graph.read().unwrap().contains_node(test_station_3));
    }
}