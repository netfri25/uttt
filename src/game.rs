use macroquad::prelude::*;

use crate::frame::Frame;
use crate::grid::Grid as _;
use crate::grid::naive::NaiveGrid;

pub type Grid = NaiveGrid;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    events: Vec<Event>,
}

impl Game {
    pub fn draw(&mut self, frame: Frame) {
        draw_grid_lines(frame, 0.02, WHITE);
    }
}

pub enum Event {}

fn draw_grid_lines(frame: Frame, thickness_percent: f32, color: Color) {
    let Rect { x, y, w, h } = *frame;

    let dx = w / 3.;
    let dy = h / 3.;
    let thickness = dx.min(dy) * thickness_percent;

    draw_line(x + dx * 1., y, x + dx * 1., y + h, thickness, color);
    draw_line(x + dx * 2., y, x + dx * 2., y + h, thickness, color);
    draw_line(x, y + dy * 1., x + w, y + dy * 1., thickness, color);
    draw_line(x, y + dy * 2., x + w, y + dy * 2., thickness, color);
}
