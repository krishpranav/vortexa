/**
 * @file: paint.rs
 * @author: Krisna Pranav
*/

use super::layout::LayoutBox;
use super::font::{get_char_bitmap, FONT_WIDTH, FONT_HEIGHT};

pub fn paint(layout: &Vec<LayoutBox>, buffer: &mut [u32], width: usize, offset_y: usize) {
    for item in layout {
        if let Some(text) = &item.text {
            let mut cursor_x = item.x;

            for c in text.chars() {
                draw_char(
                    c,
                    cursor_x,
                    item.y,
                    buffer,
                    width
                );

                cursor_x += FONT_WIDTH + 1;
            }
        }
    }
}

fn draw_char(c: char, x: usize, y: usize, buffer: &mut [u32], screen_width: usize) {
    let bitmap = get_char_bitmap(c);

    for row in 0..FONT_HEIGHT {
        let bits = bitmap[row];

        for col in 0..FONT_WIDTH {
            if bits & (1 << (7 - col)) != 0 {
                let px = x + col;
                let py = y + row;

                let index = py * screen_width + px;

                if index < buffer.len() {
                    buffer[index] = 0x000000;
                }
            }
        }
    }
}