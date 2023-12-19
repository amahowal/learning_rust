use std::net::TcpListener;

fn main() {
    // listening at port 7878 on localhost for a TCP connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
