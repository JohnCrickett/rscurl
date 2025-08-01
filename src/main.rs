use std::net::TcpStream;
use std::io::{Write, Read};
use std::error::Error;
use std::time::Duration;

use clap::Parser;
use url::{Url};

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rscurl, a simple curl clone in Rust")]
struct Arguments {
    /// Select the HTTP Method
    #[arg(
        short = 'X',
        long = "request",
        default_value = "GET",
        value_parser = ["GET", "POST", "DELETE", "PUT"]
    )]
    method: String,
    url: String,
    /// Enable verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
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

    if args.verbose {
        println!("> {} {} HTTP/1.1", args.method, parsed_url.path());
        println!("> Host: {}", parsed_url.host_str().unwrap());
        println!("> Accept: */*\n>");
    }

    match send_request(parsed_url.host_str().unwrap(), 80, parsed_url.path(), &args.method) {
        Ok(response) => {
            let parts: Vec<&str> = response.split("\r\n\r\n").collect();

            if args.verbose {
                for line in parts[0].lines() {
                    println!("< {}", line);
                }
                println!("<");
            }
            
            if parts.len() > 1 {
                for line in parts[1].lines() {
                    println!("{}", line);
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn send_request(host: &str, port: u16, path: &str, method: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect((host, port))?;

    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;

    let request = format!(
        "{} {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         User-Agent: rust-client\r\n\
         Accept: */*\r\n\
         \r\n",
        method, path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}