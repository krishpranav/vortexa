/**
 * @file: chrome.rs
 * @author: Krisna Pranav
*/

pub struct ChromeUI {
    pub toolbar_height: usize,
}

impl ChromeUI {
    pub fn new() -> Self {
        Self {
            toolbar_height: 60
        }
    }
}