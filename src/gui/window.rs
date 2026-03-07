/**
 * @file: window.rs
 * @author: Krisna Pranav
*/

use minifb::{Key, Window, WindowOptions};
use crate::engine::layout::LayoutBox;
use crate::engine::paint;
use crate::browser::chrome::ChromeUI;

pub struct WindowApp {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    chrome: ChromeUI,
}

impl WindowApp {

    pub fn new(width: usize, height: usize) -> Self {

        Self {
            width,
            height,
            buffer: vec![0xffffff; width * height],
            chrome: ChromeUI::new(),
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

            self.draw_toolbar();

            paint::paint(
                &layout,
                &mut self.buffer,
                self.width,
                self.chrome.toolbar_height
            );

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

    fn draw_toolbar(&mut self) {

        for y in 0..self.chrome.toolbar_height {
            for x in 0..self.width {

                let index = y * self.width + x;

                self.buffer[index] = 0xdddddd;
            }
        }

        for y in 20..40 {
            for x in 150..(self.width - 20) {

                let index = y * self.width + x;

                self.buffer[index] = 0xffffff;
            }
        }

        for y in 20..40 {
            for x in 20..60 {

                let index = y * self.width + x;

                self.buffer[index] = 0x999999;
            }
        }

        for y in 20..40 {
            for x in 70..110 {

                let index = y * self.width + x;

                self.buffer[index] = 0x999999;
            }
        }
    }
}