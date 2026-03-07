/**
 * @file: window.rs
 * @author: Krisna Pranav
*/

use minifb::{Key, Window, WindowOptions};

use crate::engine::layout::LayoutBox;
use crate::engine::paint;

pub struct WindowApp {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl WindowApp {

    pub fn new(width: usize, height: usize) -> Self {

        Self {
            width,
            height,
            buffer: vec![0xffffff; width * height],
        }
    }

    pub fn run(&mut self, layout: Vec<LayoutBox>) {

        let mut window = Window::new(
            "Vortexa",
            self.width,
            self.height,
            WindowOptions::default(),
        ).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {

            self.clear();

            paint::paint(&layout, &mut self.buffer, self.width);

            window.update_with_buffer(
                &self.buffer,
                self.width,
                self.height
            ).unwrap();
        }
    }

    fn clear(&mut self) {

        for pixel in self.buffer.iter_mut() {
            *pixel = 0xffffff;
        }
    }
}