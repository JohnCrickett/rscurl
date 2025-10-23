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
    /// HTTP request body (adds Content-Length and sends a body)
    #[arg(short = 'd', long = "data")]
    data: Option<String>,
    /// Add header (can be used multiple times)
    #[arg(short = 'H', long = "header")]
    headers: Vec<String>,
}

fn main() {
    let args = Arguments::parse();

    let parsed_url = match Url::parse(&args.url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error parsing url: {e}");
            std::process::exit(1);
        }
    };

    let host = match parsed_url.host_str() {
        Some(h) => h,
        None => {
            eprintln!("Error: URL must include a host");
            std::process::exit(1);
        }
    };

    let port = parsed_url.port_or_known_default().unwrap_or(80);


    let user_headers: Vec<String> = args
        .headers
        .iter()
        .map(|h| h.trim().to_string())
        .filter(|h| !h.to_lowercase().starts_with("content-length:"))
        .collect();

    let has_content_type = user_headers
        .iter()
        .any(|h| h.to_lowercase().starts_with("content-type:"));


    if args.verbose {
        println!("> {} {} HTTP/1.1", args.method, parsed_url.path());
        println!("> Host: {}", parsed_url.host_str().unwrap());
        println!("> Accept: */*");
        for h in &user_headers {
            println!("> {}", h);
        }
        if let Some(ref body) = args.data {
            let len = body.as_bytes().len();
            if !has_content_type {
                println!("> Content-Type: application/x-www-form-urlencoded");
            }
            println!("> Content-Length: {}", len);
        }
        println!(">");

    }

    match send_request(host, port, parsed_url.path(), &args.method, args.data.as_deref(), &args.headers) {
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

fn send_request(host: &str, port: u16, path: &str, method: &str, body: Option<&str>, headers: &[String]) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect((host, port))?;

    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;

    let mut request = format!(
        "{} {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         User-Agent: rscurl\r\n\
         Accept: */*\r\n",
        method, path, host
    );

    for header in headers {
        request.push_str(header);
        request.push_str("\r\n");
    }

    let has_content_type = headers
        .iter()
        .any(|header| header.to_lowercase().starts_with("content-type:"));

    if let Some(b) = body {
        // Add a default Content-Type if the user didn't provide one
        if !has_content_type {
            request.push_str("Content-Type: application/x-www-form-urlencoded\r\n");
        }
        // Always send correct Content-Length for provided body
        request.push_str(&format!("Content-Length: {}\r\n", b.as_bytes().len()));
    }
    request.push_str("\r\n");

    if let Some(b) = body {
        request.push_str(b);
    }

    request.push_str("\r\n");

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}