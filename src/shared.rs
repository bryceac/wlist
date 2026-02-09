use std::{ fs::File, io::{ self, Read }};

use url_path::UrlPath;

pub fn real_path(p: &str) -> String {
    if p.starts_with("~") {
        shellexpand::tilde(p).into_owned()
    } else {
        UrlPath::new(p).normalize()
    }
}

pub fn file_content(p: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();

    File::open(p)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}