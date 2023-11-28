use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

use rand::Rng;

fn init_database() {
    /*
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    client.simple_query("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
    ").unwrap();
    */
}

fn handle_connection_test(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes

    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer

    let request = String::from_utf8_lossy(&buffer[..]); /* convert buffer to string
                                                                 *  use to_string() to convert to
                                                                 *  String*/

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_test2(mut stream: TcpStream, mut counter: i32) {
    let mut buf_reader = BufReader::new(&mut stream);

    //let http_request = String::new();
/*
    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();*/

    //if PRINTING {println!("Request:\n{}", http_request);} // need to examine borrow errors with buf_reader

    //if PRINTING {println!("Request:\n{:#?}", http_request);} // need to examine borrow errors with buf_reader

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    //stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_new(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes
    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer
    let request = String::from_utf8_lossy(&buffer[..]); // convert buffer to string
    //let mut response: String = String::new();

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
        //let response = handle_get_request(request);
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);
    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");





    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
    stream.write_all(response.as_bytes()).unwrap();
}

fn test_bufReader() {
    // 1. Printing text file contents
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    println!("\n1. Reading file_contents:\n");
    for line in buf_reader.lines() {
        println!("{}", line.unwrap());
    }

    // 2. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let file_contents = io::read_to_string(buf_reader).unwrap();

    println!("\n2.1 Reading file_contents:\n{:?}", file_contents);
    println!("\n2.2 Reading file_contents:\n{}", file_contents);

    // 3. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut file_contents: String = String::new();

    for line in buf_reader.by_ref().lines() {
        //buf_reader.read_line(&mut file_contents);
        file_contents.push_str(line.unwrap().as_str());
        file_contents.push_str("\n");
    }

    println!("\n3.1 Reading file_contents:\n{:?}", file_contents); // printing raw string
    println!("\n3.2 Reading file_contents:\n{}", file_contents); // pretty printing with newlines
}

fn test_array_buffer() {

}
