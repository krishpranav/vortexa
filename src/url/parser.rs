/**
 * @file: parser.rs
 * @author: Krisna Pranav
*/

pub struct Url {
    pub host: String,
    pub path: String,
}

pub fn parse(url: &str) -> Result<Url, String> {
    if !url.starts_with("http://") {
        return Err("Only http supported".into());
    }

    let trimmed = &url[7..];
    let parts: Vec<&str> = trimmed.splitn(2, '/').collect();

    let host = parts[0].to_string();
    let path = if parts.len() > 1 {
        format!("/{}", parts[1])
    } else {
        "/".into()
    };

    Ok(Url { host, path })
}