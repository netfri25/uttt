use macroquad::prelude::*;

use crate::frame::Frame;
use crate::game::Game;
use crate::game::grid_draw::INNER_PAD_PERCENT;
use crate::game::grid_layout::GridLayout;

impl Game {
    pub fn update(&mut self, frame: Frame) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let (x, y) = mouse_position();

        let layout = GridLayout::new(frame).with_pad(INNER_PAD_PERCENT);
        let Some(pos) = layout.pos_at(x, y) else {
            return;
        };

        let success = self.set(pos, self.turn);
        if success {
            self.turn = self.turn.flip();
        }
    }
}
