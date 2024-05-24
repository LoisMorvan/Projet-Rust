use std::env;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let ip_server = env::var("IP_SERVER").unwrap();
    let mut stream = TcpStream::connect(&ip_server).expect("Could not connect to server");

    loop {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);

            if response.contains("It's your turn") {
                let mut input = String::new();
                println!("{}", response);
                io::stdin().read_line(&mut input).expect("Failed to read input");
                stream.write_all(input.trim().as_bytes()).expect("Failed to write to server");
            } else {
                println!("{}", response);
            }
        }
    }
}
