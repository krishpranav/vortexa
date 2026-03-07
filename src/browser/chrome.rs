/**
 * @file: chrome.rs
 * @author: Krisna Pranav
*/

pub struct ChromeUI {
    pub toolbar_height: usize,
    pub address_input: String,
    pub focused: bool,
    pub history: Vec<String>,
    pub history_index: usize,
}

impl ChromeUI {

    pub fn new() -> Self {
        Self {
            toolbar_height: 60,
            address_input: String::new(),
            focused: false,
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn navigate(&mut self, url: String) {
        self.history.push(url.clone());
        self.history_index = self.history.len() - 1;
        self.address_input = url;
    }

    pub fn back(&mut self) -> Option<String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            return Some(self.history[self.history_index].clone());
        }
        None
    }

    pub fn forward(&mut self) -> Option<String> {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            return Some(self.history[self.history_index].clone());
        }
        None
    }
}