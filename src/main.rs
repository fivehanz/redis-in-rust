// Uncomment this block to pass the first stage
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_stream(stream) {
                Ok(_) => {}
                Err(_e) => {}
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(_stream: TcpStream) -> Result<(), std::io::Error> {
    // read from stream
    // let buf_reader = BufReader::new(stream.try_clone()?);
    // ! TODO create a resp decoder and encoder lib with the buf reader and writer
    todo!();
    // let mut decoder = Decoder::new(buf_reader);

    // read into buffer
    // match stream.read(&mut buf) {
    //     Ok(bytes) => {
    //         print!("read {} bytes: {:?}", bytes, &buf[..bytes]);
    //     }
    //     // handle stream read error
    //     Err(e) => {
    //         println!("error: {}", e);
    //     }
    // }

    // print!("read resp: {:?}", decoder.decode().unwrap());

    // write bytes to the stream
    // stream.write("PONG".as_bytes()).unwrap();

    // shutdown
    // Ok(stream.shutdown(Shutdown::Both).unwrap())
}

// ping handler
fn _handle_ping() {
    // PONG
    todo!()
}
