use crate::frame::Frame;

pub mod consts;
pub mod grid_layout;
mod impls;

pub trait Draw {
    fn draw(&self, frame: Frame);
}
