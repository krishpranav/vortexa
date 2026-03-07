/**
 * @file: parser.rs
 * @author: Krisna Pranav
*/

pub struct Url {
    pub scheme: String,
    pub host: String,
    pub path: String,
}

pub fn parse(input: &str) -> Result<Url, String> {

    let (scheme, rest) = if input.starts_with("https://") {
        ("https", &input[8..])
    } else if input.starts_with("http://") {
        ("http", &input[7..])
    } else {
        return Err("Unsupported scheme".into());
    };

    let parts: Vec<&str> = rest.splitn(2, '/').collect();

    let host = parts[0].to_string();

    let path = if parts.len() > 1 {
        format!("/{}", parts[1])
    } else {
        "/".to_string()
    };

    Ok(Url {
        scheme: scheme.to_string(),
        host,
        path,
    })
}