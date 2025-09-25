mod config; mod math; mod input; mod render; mod entities; mod game;

use std::time::{Duration, Instant};
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

use config::{WIDTH, HEIGHT};
use game::Game;
use input::Input;
use math::Vec2;

fn main() {
    let mut window = Window::new(
        "Wave Survival — Rust (no engine)",
        WIDTH,
        HEIGHT,
        WindowOptions { resize: false, scale: Scale::X1, ..WindowOptions::default() },
    ).expect("Unable to open window");

    window.limit_update_rate(Some(Duration::from_micros(16600))); // ~60 FPS

    let mut fb: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut game = Game::default();

    let mut input = Input::default();
    let mut prev = Instant::now();

    while window.is_open() {
        // dt
        let now = Instant::now();
        let mut dt = (now - prev).as_secs_f32();
        prev = now;
        if dt > 0.05 { dt = 0.05; }

        // input polling
        input.w = window.is_key_down(Key::W) || window.is_key_down(Key::Up);
        input.s = window.is_key_down(Key::S) || window.is_key_down(Key::Down);
        input.a = window.is_key_down(Key::A) || window.is_key_down(Key::Left);
        input.d = window.is_key_down(Key::D) || window.is_key_down(Key::Right);
        input.key_enter = window.is_key_down(Key::Enter);
        input.mouse_left = window.get_mouse_down(MouseButton::Left);
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) { input.mouse_pos = Vec2 { x: mx, y: my }; }

        // update & draw
        game.update(dt, &input);
        game.draw(&mut fb);
        window.set_title(&game.window_title());
        window.update_with_buffer(&fb, WIDTH, HEIGHT).unwrap();
    }
}
