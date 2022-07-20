use std::{net, io::{Write, Read}};

const LOCALHOST: &str = "127.0.0.1";

trait TrimBlankCharacters {
    fn clean(&mut self) -> &mut String;
}

impl TrimBlankCharacters for String {

    fn clean(&mut self) -> &mut String {
        fn is_blank(ch: char) -> bool {
            return ch == '\n' || ch == '\r' || ch.is_whitespace() || ch == '\0';
        }

        while self.ends_with(is_blank) {
            self.pop();
        }
        while self.starts_with(is_blank) {
            self.remove(0);            
        }
        self
    }
}

fn handle_stream(mut stream: net::TcpStream) {
    let mut read_buffer = [0; 1024];
                
    stream.read(&mut read_buffer).unwrap();

    let mut payload = String::from(String::from_utf8_lossy(&read_buffer[..]));
    payload.clean();

    let response: &str = if payload.eq("PING") {
        "PONG\n"
    } else {
        "That wasn't a ping!\n"
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn ping_pong_server() {
    let port = "7878";
    let addr = format!("{}:{}", LOCALHOST, port);

    let listener = net::TcpListener::bind(addr).unwrap();

    for possible_stream in listener.incoming() {
        match possible_stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => println!("Error encountered : {}", e.to_string())
        }
    }
}

fn main() {
    ping_pong_server();
}
