extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::video::Window;

fn colour_pixels(pixels: &mut Vec<u8>) {
    for pixel in pixels {
        *pixel = 100;
    }
}

fn update_window(canvas: &mut Canvas<Window>) {
    let viewport = canvas.viewport();
    let width = viewport.width();
    let height = viewport.height();
    let pitch = width * 4;
    let bytes = pitch * height;

    //let pixels = vec![0u8; bytes as usize].into_boxed_slice();
    let mut pixels = vec![199u8; bytes as usize];
    colour_pixels(&mut pixels);

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)
        .unwrap();
    texture.update(None, &pixels[..], pitch as usize).unwrap();

    canvas.clear();
    canvas.present();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rusty Hero", 500, 500)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::Window {
                    win_event: WindowEvent::SizeChanged(0, 0),
                    ..
                } => {
                    update_window(&mut canvas);
                }
                _ => {}
            }
        }
    }
}
