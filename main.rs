
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    //// create listener
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").expect("could not connect");

    ///// looop for listener + create many threads server
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // run many thread
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("error connection: {}", e)
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        // craete buffer for data
        let mut buffer = [0; 1024];
        // read data
        stream.read(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nServer run";
        // send data
        stream.write(response.as_bytes()).unwrap();
        // clear stream + handle
        stream.flush().unwrap();
    }
}
