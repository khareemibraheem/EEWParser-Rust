use std::fs;
use std::io::{self, Read};

pub fn read_content_from_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn read_content_from_stdin() -> io::Result<String> {
    let mut raw: Vec<u8> = Vec::new();
    let mut buf = [0u8; 1];
    let mut stdin = io::stdin();

    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        }

        raw.push(buf[0]);

        if buf[0] == b'=' {
            break;
        }
    }

    Ok(String::from_utf8_lossy(&raw).to_string())
}
