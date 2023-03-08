use std::borrow::Cow;
use macroquad::prelude::*;
use macroquad::window::*;
use macroquad::ui::*;
use miniquad::graphics::*;
use petgraph::stable_graph::NodeIndex;
use macroquad::ui::Id;
use crate::widgets::Group;


pub struct Data {
    pub inventory: Vec<String>,
}
impl Data {
    pub fn new() -> Data {
        Data {
            inventory: vec![],
        }
    }

    pub fn inventory(&mut self, ui: &mut Ui) {
        for (n, item) in self.inventory.iter().enumerate() {
            let drag = Group::new(hash!("inventory", n), Vec2::new(395., 50.))
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &item);
                });

        }
    }

}

struct station_coordinates {
    node_index: NodeIndex,
    x: f32,
    y: f32,
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RlwyNetSim".to_string(),
        window_height: 1500,
        window_width: 1500,
        fullscreen: false,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

pub fn draw_station(ui: &mut Ui)  {
    let (mouse_x, mouse_y) = mouse_position();
    draw_circle(mouse_x, mouse_y, 60., RED);

}

