use macroquad::prelude::*;

use uttt_core::grid::Grid;
use uttt_core::grid::Pos;
use uttt_core::grid::Winner;
use uttt_core::grid::naive::NaiveGrid;
use uttt_core::grid::section_tracker::SectionTracker;
use uttt_core::grid::winner_cache::WinnerCache;
use uttt_core::player::Player;
use uttt_core::subgrid::Subgrid;

use crate::draw::Draw;
use crate::draw::consts;
use crate::draw::grid_layout::GridLayout;
use crate::frame::Frame;

impl<G> Draw for SectionTracker<G>
where
    G: Draw,
{
    fn draw(&self, frame: Frame) {
        let layout = GridLayout::new(frame);
        for (outer_index, frame) in layout.grid_frame_iter() {
            if self.section().is_some_and(|allowed| allowed != outer_index) {
                draw_rectangle(frame.x, frame.y, frame.w, frame.h, WHITE.with_alpha(0.3));
            }
        }

        G::draw(self, frame)
    }
}

impl<G> Draw for WinnerCache<G>
where
    G: Draw + Grid,
{
    fn draw(&self, frame: Frame) {
        draw_grid_lines(frame, consts::LINE_WIDTH_PERCENT, WHITE);

        let layout = GridLayout::new(frame);
        for (outer, frame) in layout.grid_frame_iter() {
            let frame = frame.shrink(consts::INNER_PAD_PERCENT);

            if let Winner::Player(player) = self.winner_of(outer) {
                player.draw(frame);
                continue;
            }

            draw_grid_lines(frame, consts::LINE_WIDTH_PERCENT, WHITE);

            let layout = GridLayout::new(frame);
            for (inner, frame) in layout.grid_frame_iter() {
                let pos = Pos { outer, inner };
                if let Some(player) = self.get(pos) {
                    player.draw(frame);
                }
            }
        }
    }
}

impl Draw for NaiveGrid {
    fn draw(&self, frame: Frame) {
        draw_grid_lines(frame, consts::LINE_WIDTH_PERCENT, WHITE);

        let layout = GridLayout::new(frame);
        for (outer, frame) in layout.grid_frame_iter() {
            let frame = frame.shrink(consts::INNER_PAD_PERCENT);

            let grid = self.grid_at(outer);
            draw_grid_lines(frame, consts::LINE_WIDTH_PERCENT, WHITE);

            let layout = GridLayout::new(frame);
            for (inner, frame) in layout.grid_frame_iter() {
                if let Some(player) = grid.get(inner) {
                    player.draw(frame);
                }
            }
        }
    }
}

impl Draw for Player {
    fn draw(&self, frame: Frame) {
        let frame = frame.shrink(consts::PLAYER_PAD_PERCENT);
        let thickness_percent = consts::PLAYER_THICK_PERCENT;

        match self {
            Player::X => draw_x(frame, thickness_percent, RED),
            Player::O => draw_o(frame, thickness_percent, BLUE),
        }
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
