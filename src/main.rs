/**
 * @file: main.rs
 * @author: Krisna Pranav
*/

mod gui;
mod browser;
mod network;
mod url;
mod engine;

use gui::window::WindowApp;
use engine::{html, layout};
use network::http;
use url::parser;

fn main() {

    let url = "https://google.com";

    let parsed = parser::parse(url).unwrap();
    let response = http::fetch(&parsed.host, &parsed.path).unwrap();

    let body = http::extract_body(&response);

    let dom = html::parse(&body);
    let layout_tree = layout::build_layout(&dom, 800);

    let mut app = WindowApp::new(800, 600, layout_tree);

    app.run();
}