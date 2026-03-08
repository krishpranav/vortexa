/**
 * @file: http.rs
 * @author: Krisna Pranav
*/

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;
use crate::url::parser::Url;

pub fn fetch(url: &Url) -> Result<String, String> {
    if url.scheme == "http" {
        fetch_http(url)
    } else {
        fetch_https(url)
    }
}

pub fn fetch_http(url: &Url) -> Result<String, String> {
    let mut stream = TcpStream::connect(format!("{}:80", host))
        .map_err(|e| e.to_string())?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path, url.host
    );

    stream.write_all(request.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut response = String::new();

    stream.read_to_string(&mut response)
        .map_err(|e| e.to_string())?;

    Ok(response)
}

fn fetch_https(url: &Url) -> Result<String, String> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(TLS_SERVER_ROOTS.iter().cloned());
}

pub fn extract_body(response: &str) -> String {
    if let Some(index) = response.find("\r\n\r\n") {
        return response[index + 4..].to_string();
    }

    response.to_string()
}