/**
 * @file: main.rs
 * @author: Krisna Pranav
*/

mod gui;
mod network;
mod url;
mod engine;

use gui::window::WindowApp;

fn main() {
    let mut app = WindowApp::new(800, 600);

    let url = "http://example.com";

    let parsed = url::parser::parse(url).unwrap();
    let response = network::http::fetch(&parsed.host, &parsed.path).unwrap();
    let body = network::http::extract_body(&response);

    let dom = engine::html::parse(&body);
    let layout = engine::layout::build_layout(&dom, 800);

    app.run(layout);
}