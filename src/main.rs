use clap::Parser;
use url::{Url, ParseError};

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

}
