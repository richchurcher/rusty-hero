extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;

fn blank(renderer: &mut sdl2::render::Renderer, is_white: bool) {
    match is_white {
        true => renderer.set_draw_color(Color::RGB(92, 12, 12)),
        false => renderer.set_draw_color(Color::RGB(255, 255, 255)),
    }
    renderer.clear();
    renderer.present();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rusty Hero", 200, 200)
        .resizable()
        .build()
        .unwrap();

    let mut is_white = true;
    let mut renderer = window.renderer().build().unwrap();
    blank(&mut renderer, is_white);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game
                },
                Event::Window { win_event_id: WindowEventId::Resized, .. } => {
                    blank(&mut renderer, is_white);
                    is_white = !is_white;
                },
                _ => {}
            }
        }
    }
}
