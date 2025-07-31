use std::net::TcpStream;
use std::io::{Write, Read};
use std::error::Error;
use std::time::Duration;

use clap::Parser;
use url::{Url};

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rscurl, a simple curl clone in Rust")]
struct Arguments {
    #[arg(
        short = 'X',
        long = "request",
        default_value = "GET",
        value_parser = ["GET", "POST", "DELETE", "PUT"]
    )]
    method: String,
    url: String,
}

fn main() {
    let args = Arguments::parse();
    println!("connecting to: {}", args.url);

    let parsed_url = match Url::parse(&args.url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error parsing url: {e}");
            std::process::exit(1);
        }
    };

    println!("Sending Request {} {} HTTP/1.1", args.method, parsed_url.path());
    println!("Host: {}", parsed_url.host_str().unwrap());
    println!("Accept: */*");

    match send_request(parsed_url.host_str().unwrap(), 80, parsed_url.path()) {
        Ok(response) => {
            println!("Received response:");
            println!("{}", response);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn send_request(host: &str, port: u16, path: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect((host, port))?;

    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         User-Agent: rust-client\r\n\
         Accept: */*\r\n\
         \r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}
