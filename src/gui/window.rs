/**
 * @file: window.rs
 * @author: Krisna Pranav
*/

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};

use crate::engine::layout::LayoutBox;
use crate::engine::paint;
use crate::browser::chrome::ChromeUI;
use crate::network::http;
use crate::url::parser;
use crate::engine::{html, layout};

pub struct WindowApp {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    chrome: ChromeUI,
    layout: Vec<LayoutBox>,
}

impl WindowApp {

    pub fn new(width: usize, height: usize, layout: Vec<LayoutBox>) -> Self {

        Self {
            width,
            height,
            buffer: vec![0xffffff; width * height],
            chrome: ChromeUI::new(),
            layout,
        }
    }

    pub fn run(&mut self) {

        let mut window = Window::new(
            "Vortexa",
            self.width,
            self.height,
            WindowOptions::default(),
        ).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {

            self.handle_mouse(&window);
            self.handle_keyboard(&window);

            self.clear();
            self.draw_toolbar();

            paint::paint(
                &self.layout,
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

    fn handle_mouse(&mut self, window: &Window) {

        if window.get_mouse_down(MouseButton::Left) {

            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {

                let x = x as usize;
                let y = y as usize;

                if y > 20 && y < 40 && x > 150 && x < self.width - 20 {
                    self.chrome.focused = true;
                }

                if y > 20 && y < 40 && x > 20 && x < 60 {
                    if let Some(url) = self.chrome.back() {
                        self.load_page(&url);
                    }
                }

                if y > 20 && y < 40 && x > 70 && x < 110 {
                    if let Some(url) = self.chrome.forward() {
                        self.load_page(&url);
                    }
                }
            }
        }
    }

    fn handle_keyboard(&mut self, window: &Window) {

        if !self.chrome.focused {
            return;
        }

        let keys = window.get_keys_pressed(KeyRepeat::Yes);

        for key in keys {
            match key {
                Key::Enter => {
                    let url = self.chrome.address_input.clone();
                    self.load_page(&url);
                }

                Key::Backspace => {
                    self.chrome.address_input.pop();
                }

                _ => {
                    if let Some(c) = key_to_char(key) {
                        self.chrome.address_input.push(c);
                    }
                }
            }
        }
    }

    fn load_page(&mut self, url: &str) {

        if let Ok(parsed) = parser::parse(url) {

            if let Ok(response) = http::fetch(&parsed.host, &parsed.path) {

                let body = http::extract_body(&response);

                let dom = html::parse(&body);

                self.layout = layout::build_layout(&dom, self.width);

                self.chrome.navigate(url.to_string());
            }
        }
    }

    fn draw_toolbar(&mut self) {

        for y in 0..self.chrome.toolbar_height {
            for x in 0..self.width {
                self.buffer[y * self.width + x] = 0xdddddd;
            }
        }

        for y in 20..40 {
            for x in 150..(self.width - 20) {
                self.buffer[y * self.width + x] = 0xffffff;
            }
        }

        for y in 20..40 {
            for x in 20..60 {
                self.buffer[y * self.width + x] = 0x999999;
            }
        }

        for y in 20..40 {
            for x in 70..110 {
                self.buffer[y * self.width + x] = 0x999999;
            }
        }
    }
}

fn key_to_char(key: Key) -> Option<char> {

    match key {

        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),

        Key::Key0 => Some('0'),
        Key::Key1 => Some('1'),
        Key::Key2 => Some('2'),
        Key::Key3 => Some('3'),
        Key::Key4 => Some('4'),
        Key::Key5 => Some('5'),
        Key::Key6 => Some('6'),
        Key::Key7 => Some('7'),
        Key::Key8 => Some('8'),
        Key::Key9 => Some('9'),

        Key::Period => Some('.'),
        Key::Slash => Some('/'),
        Key::Minus => Some('-'),

        _ => None
    }
}