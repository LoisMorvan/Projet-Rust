use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to server");

    loop {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);

            if response.trim() == "OK" {
                println!("You guessed the correct number!");
                break;
            } else if response.contains("Game Over") {
                println!("{}", response);
                break;
            } else if response.contains("It's your turn") {
                let mut input = String::new();
                println!("Enter your guess:");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                stream.write(input.as_bytes()).expect("Failed to write to server");
            } else if response.contains("Waiting for another player") {
                println!("{}", response);
                std::thread::sleep(std::time::Duration::from_secs(1));
            } else if response.contains("You are in the lobby") {
                println!("{}", response);
            }
        }
    }
}
