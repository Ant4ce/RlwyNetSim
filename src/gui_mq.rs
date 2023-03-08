use std::borrow::Cow;
use macroquad::prelude::*;
use macroquad::window::*;
use macroquad::ui::*;
use miniquad::graphics::*;
use petgraph::stable_graph::NodeIndex;
use macroquad::ui::Id;

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

pub fn draw_station(ui: &mut Ui)  {
    let (mouse_x, mouse_y) = mouse_position();
    draw_circle(mouse_x, mouse_y, 60., RED);
    let fake_id : Id = 50 as u64;
    let my_vec: Option<Vec2> = Some(Vec2::new(0 as f32, 300 as f32));
    let my_string: Cow<'_, str> = Cow::Owned("holllaaaaa".to_string());
    let button_return = ui.button( my_vec,  UiContent::Label(my_string));
}

