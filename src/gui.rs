use std::borrow::Cow;
use std::sync::{Arc, Mutex, RwLock};
use petgraph::stable_graph::NodeIndex;
use petgraph::prelude::StableGraph;
use crate::graph::*;
use crate::route::Route;
use crate::station::Station;
use crate::train::TrainType::*;





struct station_collection {
    instances: Vec<station_coordinates>,
}

struct station_coordinates {
    node_index: NodeIndex,
    x: f32,
    y: f32,
}

// draw all the stations inside the station_coordinates
pub fn draw_stations() {

}
struct Station_UI {
    name: String,
    n_freight: u32,
    n_lowspeed: u32,
    n_highspeed: u32,
}
impl Default for Station_UI {
    fn default() -> Self {
        Self {
            name: "Berlin".to_owned(),
            n_freight: 5,
            n_lowspeed: 5,
            n_highspeed: 5,
        }
    }
}
impl Station_UI {
    fn make_station(&mut self, ui: &mut egui::Ui) {
        ui.heading("Create Station");

        ui.horizontal(|ui| {
            ui.label("Station Name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.label("Number of Platforms");
        egui::Slider::new(&mut self.n_freight, 0..=100).text("Freight Platforms");
        egui::Slider::new(&mut self.n_lowspeed, 0..=100).text("LowSpeed Platforms");
        egui::Slider::new(&mut self.n_highspeed, 0..=100).text("HighSpeed Platforms");
        ui.label(format!("Your Station: Name '{}', # of platforms: {}", self.name, self.n_highspeed + self.n_lowspeed + self.n_freight));
    }
}
// add the stations to the station_coordinates
pub fn build_station(x_coordinate: f32, y_coordinate: f32)  {

   // let station = station_coordinates {
   //     node_index: ,
   //     x: x_coordinate,
   //     y: y_coordinate,
   // }
   //
   // draw_circle(x, y, 20., RED);

}

