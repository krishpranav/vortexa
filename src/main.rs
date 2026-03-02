mod network;
mod url;
mod engine;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: vortexa http://example.com");
        return;
    }

    let input_url = &args[1];

    let parsed = match url::parser::parse(input_url) {
        Ok(u) => u,
        Err(e) => {
            println!("URL Error: {}", e);
            return;
        }
    };

    let response = match network::http::fetch(&parsed.host, &parsed.path) {
        Ok(r) => r,
        Err(e) => {
            println!("Network Error: {}", e);
            return;
        }
    };

    let body = network::http::extract_body(&response);

    let dom = engine::html::parse(&body);

    engine::render::render(&dom);
}