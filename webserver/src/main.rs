use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
//use std::io::prelude::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream=stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){

    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    //println!("req: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get)
        {
            ("HTTP/1.1 200 OK","index.html") 
        }else{
            ("HTTP/1.1 404 NOT FOUND","404.html")
        };

    // if buffer.starts_with(get){

    //     let content = fs::read_to_string("index.html").unwrap();
    //     let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",content.len(), content);
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();

    // }else{

    //     let status_line= "HTTP/1.1 404 NOT FOUND";
    //     let content = fs::read_to_string("404.html").unwrap();
    //     let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status_line,content.len(), content);
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    //let status_line= "HTTP/1.1 404 NOT FOUND";
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status_line,content.len(), content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}