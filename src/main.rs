extern crate sdl2;

use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;
use sdl2::render::{Renderer, TextureAccess};

fn update_window(renderer: &mut Renderer) {
    let viewport = renderer.viewport();
    let texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::ARGB8888, TextureAccess::Streaming, viewport.width(), viewport.height());
    renderer.clear();
    renderer.present();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rusty Hero", 500, 500)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game
                },
                Event::Window { win_event_id: WindowEventId::SizeChanged, .. } => {
                    update_window(&mut renderer);
                },
                _ => {}
            }
        }
    }
}
