/**
 * @file: window.rs
 * @author: Krisna Pranav
*/

use minifb::{Key, Window, WindowOptions};
use crate::engine::layout::LayoutBox;

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
        )
            .unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.clear();

            for box_item in &layout {
                self.draw_rect(box_item.x, box_item.y, box_item.width, box_item.height);
            }

            window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }

    fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0xffffff;
        }
    }

    fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for yy in y..(y + h).min(self.height) {
            for xx in x..(x + w).min(self.width) {
                let idx = yy * self.width + xx;
                self.buffer[idx] = 0x000000;
            }
        }
    }
}