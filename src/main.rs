use macroquad::prelude::*;

mod grid;
mod player;
mod game;

fn main() {
    macroquad::Window::from_config(config(), run());
}

async fn run() {
    loop {
        next_frame().await
    }
}

fn config() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "UTTT".into(),
            window_width: 960,
            window_height: 720,
            fullscreen: false,
            sample_count: 2,
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
            ..Default::default()
        }),

        default_filter_mode: FilterMode::Linear,
        ..Default::default()
    }
}
