/**
 * @file: http.rs
 * @author: Krisna Pranav
*/

use std::io::{Read, Write};
use std::net::TcpStream;

pub fn fetch(host: &str, path: &str) -> Result<String, String> {
    let mut stream = TcpStream::connect(format!("{}:80", host))
        .map_err(|e| e.to_string())?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );

    stream.write_all(request.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut response = String::new();

    stream.read_to_string(&mut response)
        .map_err(|e| e.to_string())?;

    Ok(response)
}

pub fn extract_body(response: &str) -> String {
    if let Some(index) = response.find("\r\n\r\n") {
        return response[index + 4..].to_string();
    }

    response.to_string()
}