// TODO: make a Draw trait to allow for a more generic approach to drawing grids
// TODO: maybe having another trait for a subgrid will be nice

use macroquad::prelude::*;

use crate::frame::Frame;

mod frame;
mod game;
mod grid;
mod player;

fn main() {
    macroquad::Window::from_config(config(), run());
}

async fn run() {
    let mut game = game::Game::default();

    loop {
        clear_background(BLACK);
        let frame = get_window_frame().to_square();
        game.update(frame);
        game.draw(frame);
        next_frame().await
    }
}

fn get_window_frame() -> Frame {
    let w = screen_width();
    let h = screen_height();
    Frame::new(0., 0., w, h)
}

fn config() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "UTTT".into(),
            window_width: 960,
            window_height: 720,
            fullscreen: false,
            sample_count: 8,
            window_resizable: true,
            platform: miniquad::conf::Platform {
                linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
                blocking_event_loop: true,
                sleep_interval_ms: Some(500),
                ..Default::default()
            },
            ..Default::default()
        },

        update_on: Some(macroquad::conf::UpdateTrigger {
            key_down: true,
            mouse_down: true,
            mouse_up: true,
            mouse_motion: true,
            ..Default::default()
        }),

        default_filter_mode: FilterMode::Linear,
        ..Default::default()
    }
}
