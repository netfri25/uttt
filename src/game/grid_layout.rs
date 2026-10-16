use macroquad::prelude::vec2;

use crate::frame::Frame;
use crate::grid::{GridIndex, Pos};

pub struct GridLayout {
    frame: Frame,
    pad: f32,
}

impl GridLayout {
    pub fn new(frame: Frame) -> Self {
        Self { frame, pad: 0. }
    }

    pub fn with_pad(mut self, pad: f32) -> Self {
        self.pad = pad;
        self
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn grid_frame_iter(&self) -> impl Iterator<Item = (GridIndex, Frame)> + 'static {
        let pad = self.pad;

        let indices = GridIndex::ALL.into_iter();
        let frames = self
            .frame
            .split_h(2)
            .flat_map(move |frame| frame.split_w(2).map(move |frame| frame.shrink(pad)));

        indices.zip(frames)
    }

    pub fn frame_in_index(&self, index: GridIndex) -> Frame {
        let index = index.get();
        let row = index / 3;
        let col = index % 3;

        let x = self.frame.x;
        let y = self.frame.y;
        let w = self.frame.w / 3.;
        let h = self.frame.h / 3.;

        Frame::new(x + w * col as f32, y + h * row as f32, w, h)
    }

    pub fn frame_in_pos(&self, pos: Pos) -> Frame {
        let outer_frame = self.frame_in_index(pos.outer).shrink(self.pad);
        Self::new(outer_frame).frame_in_index(pos.inner)
    }

    pub fn pos_at(&self, x: f32, y: f32) -> Option<Pos> {
        let outer = self.grid_index_at(x, y)?;
        let inner_frame = self.frame_in_index(outer).shrink(self.pad);
        let inner_layout = Self::new(inner_frame);
        let inner = inner_layout.grid_index_at(x, y)?;
        Some(Pos { outer, inner })
    }

    pub fn grid_index_at(&self, x: f32, y: f32) -> Option<GridIndex> {
        if !self.frame.contains(vec2(x, y)) {
            return None;
        }

        let x = x - self.frame.x;
        let y = y - self.frame.y;

        let col = (3. * x / self.frame.w).floor() as u8;
        let row = (3. * y / self.frame.h).floor() as u8;
        GridIndex::try_from(row * 3 + col).ok()
    }
}
