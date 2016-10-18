extern crate sdl2;

use sdl2::messagebox::{MESSAGEBOX_INFORMATION, show_simple_message_box};

pub fn main() {
    assert!(show_simple_message_box(MESSAGEBOX_INFORMATION,
        "Handmade Hero", "Welcome to Handmade Hero.", None).is_ok());
}
