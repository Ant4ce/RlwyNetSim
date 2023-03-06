use macroquad::prelude::*;
use macroquad::window::*;
use macroquad::ui::*;
use miniquad::graphics::*;
use petgraph::stable_graph::NodeIndex;

struct station_coordinates {
    node_index: NodeIndex,
    x: f32,
    y: f32,
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RlwyNetSim".to_string(),
        window_height: 500,
        window_width: 600,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

pub fn draw_station() -> NodeIndex {
    let (mouse_x, mouse_y) = mouse_position();
    draw_circle(mouse_x, mouse_y, 60., RED);
    let my_ui = Ui::new(&mut GraphicsContext::new(), screen_width(), screen_height());
    let my_vec = Vec2::new(200 as f32, 150 as f32);
    my_ui.popup(0, my_vec, Ui::checkbox(&mut macroquad::ui::Ui ,1, "hola", &mut false ))
}
