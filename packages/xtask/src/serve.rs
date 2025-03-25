use std::io::{BufRead, BufReader, Write as _};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::{fs, thread};

use httparse::{EMPTY_HEADER, Request};

use crate::MyResult;

const PORT: u16 = 8_000;

pub fn run() -> MyResult<()> {
    let ip_addr = Ipv4Addr::LOCALHOST;
    let listener = TcpListener::bind((ip_addr, PORT))?;

    eprintln!("listening on {ip_addr}:{PORT}");
    for res in listener.incoming() {
        let stream = res?;
        thread::spawn(move || {
            let _ = handle_connection(stream);
        });
    }

    Ok(())
}

// KISS web server: one request per connection
// (this is for local webapp development, not a C10K web server)
fn handle_connection(mut stream: TcpStream) -> MyResult<()> {
    let mut buf = Vec::new();
    let mut buf_reader = BufReader::new(&stream);
    loop {
        buf_reader.read_until(b'\n', &mut buf)?;

        let len = buf.len();
        if len < 4 {
            continue;
        }

        if buf[len - 4..] == *b"\r\n\r\n" {
            break;
        }
    }

    let mut headers = [EMPTY_HEADER; 32];
    let mut req = Request::new(&mut headers);

    if req.parse(&buf)?.is_complete()
        && req.method == Some("GET")
        && req.version == Some(1)
        && req.path.is_some()
    {
        let dist_dir = crate::repo_root().join("dist");

        match req.path.unwrap() {
            "/" | "/index.html" => {
                let body = fs::read(dist_dir.join("index.html"))?;
                respond(&mut stream, "text/html; charset=utf-8", &body)?;
            }

            "/style.css" => {
                let body = fs::read(dist_dir.join("style.css"))?;
                respond(&mut stream, "text/css; charset=utf-8", &body)?;
            }

            "/app.js" => {
                let body = fs::read(dist_dir.join("app.js"))?;
                respond(&mut stream, "text/javascript; charset=utf-8", &body)?;
            }

            "/app.wasm" => {
                let body = fs::read(dist_dir.join("app.wasm"))?;
                respond(&mut stream, "application/wasm", &body)?;
            }

            _ => {
                stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        }
    } else {
        stream.write_all(b"HTTP/1.1 403 Forbidden\r\n\r\n")?;
    }

    Ok(())
}

fn respond(stream: &mut TcpStream, content_type: &str, body: &[u8]) -> MyResult<()> {
    stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: ")?;
    stream.write_all(content_type.as_bytes())?;
    stream.write_all(b"\r\nContent-Length: ")?;
    stream.write_all(body.len().to_string().as_bytes())?;
    stream.write_all(b"\r\n\r\n")?;
    stream.write_all(body)?;
    Ok(())
}
