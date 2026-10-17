use macroquad::prelude::*;

use crate::frame::Frame;
use crate::game::Game;
use crate::game::grid_layout::GridLayout;
use crate::grid::section_tracker::SectionTracker;
use crate::grid::{Grid, GridIndex, Pos, Winner};
use crate::player::Player;

pub const INNER_PAD_PERCENT: f32 = 0.1;
pub const LINE_WIDTH_PERCENT: f32 = 0.02;
pub const PLAYER_THICK_PERCENT: f32 = 0.1;
pub const PLAYER_PAD_PERCENT: f32 = 0.4;

impl Game {
    pub fn draw(&mut self, frame: Frame) {
        if let Some(player) = self.winner {
            draw_player(frame, player);
        } else {
            draw_section_tracker(frame, &self.grid);
        }
    }
}

fn draw_section_tracker<G: Grid>(frame: Frame, grid: &SectionTracker<G>) {
    let layout = GridLayout::new(frame);
    for (outer_index, frame) in layout.grid_frame_iter() {
        if grid.section().is_some_and(|allowed| allowed != outer_index) {
            draw_rectangle(frame.x, frame.y, frame.w, frame.h, WHITE.with_alpha(0.3));
        }
    }

    draw_grid(frame, grid)
}

fn draw_grid(frame: Frame, grid: &dyn Grid) {
    draw_grid_lines(frame, LINE_WIDTH_PERCENT, WHITE);

    let layout = GridLayout::new(frame);
    for (outer_index, frame) in layout.grid_frame_iter() {
        draw_inner_grid(frame.shrink(INNER_PAD_PERCENT), outer_index, grid);
    }
}

// fn draw_grid_lines(frame: Frame) {
//     draw_grid_lines(frame, LINE_WIDTH_PERCENT, WHITE);
// }

fn draw_inner_grid(frame: Frame, outer: GridIndex, grid: &dyn Grid) {
    if let Winner::Player(player) = grid.winner_of(outer) {
        draw_player(frame, player);
        return;
    }

    draw_grid_lines(frame, LINE_WIDTH_PERCENT, WHITE);

    let layout = GridLayout::new(frame);
    for (inner, frame) in layout.grid_frame_iter() {
        let pos = Pos { outer, inner };
        if let Some(player) = grid.get(pos) {
            draw_player(frame, player);
        }
    }
}

fn draw_player(frame: Frame, player: Player) {
    let frame = frame.shrink(PLAYER_PAD_PERCENT);
    let thickness_percent = PLAYER_THICK_PERCENT;

    match player {
        Player::X => draw_x(frame, thickness_percent, RED),
        Player::O => draw_o(frame, thickness_percent, BLUE),
    }
}

fn draw_x(frame: Frame, thickness_percent: f32, color: Color) {
    let thickness = frame.w * thickness_percent;
    let l = frame.left();
    let r = frame.right();
    let t = frame.top();
    let b = frame.bottom();
    draw_line(l, t, r, b, thickness, color);
    draw_line(r, t, l, b, thickness, color);
}

fn draw_o(frame: Frame, thickness_percent: f32, color: Color) {
    let thickness = frame.w * thickness_percent;
    let Vec2 { x, y } = frame.center();
    let r = frame.w / 2.;
    let sides = 32;
    draw_poly_lines(x, y, sides, r, 0., thickness, color);
}

fn draw_grid_lines(frame: Frame, thickness_percent: f32, color: Color) {
    let Rect { x, y, w, h } = *frame;

    let dx = w / 3.;
    let dy = h / 3.;
    let thickness = dx * thickness_percent;

    draw_line(x + dx * 1., y, x + dx * 1., y + h, thickness, color);
    draw_line(x + dx * 2., y, x + dx * 2., y + h, thickness, color);
    draw_line(x, y + dy * 1., x + w, y + dy * 1., thickness, color);
    draw_line(x, y + dy * 2., x + w, y + dy * 2., thickness, color);
}
