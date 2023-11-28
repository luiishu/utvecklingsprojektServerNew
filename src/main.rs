#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

#[allow(unused)]
use std::{fs::File, thread::{self, Thread}};
#[allow(unused)]
use rand::Rng;

#[allow(unused)]
use rusqlite::{Connection, params, Result};
//use server::database::database::hello_from_database;

use crate::{init::*, request_handler::handle_request};
use crate::get_handler::*;
#[allow(unused)]
use crate::post_handler::*;
use crate::request_line::*;
use crate::database::database::*;

mod init;
mod request_handler;
mod get_handler;
mod post_handler;
mod request_line;
mod database;
mod fetch_handler;
//mod database/hello_from_database;
//static mut counter: i32 = 0;

const LAN: bool = false;
const PRINTING: bool = true;

#[allow(unused)]
const LOCALHOST_IP_V4: &str = "127.0.0.1";
#[allow(unused)]
const SERVER_IP_V4: &str = "192.168.1.178";

#[allow(unused)]
const RUST_PORT: u16 = 7878;

//static conn: Connection = Connection::open_in_memory().unwrap();

fn main() {
    hello();
    hello_from_database();
    println!("\nInitializing server...");

    let listener = TcpListener::from(if LAN {
        server::init_server_on_lan().unwrap_or_else(|err| {
            println!("Problem initializing server: {err}");
            process::exit(1);
        })
    } else {
        server::init_server_on_localhost().unwrap_or_else(|err| {
            println!("Problem initializing server: {err}");
            process::exit(1);
        })
    });

    //println!("Initializing database...");
    let conn = init_database(true);

    //test_bufReader();

    println!("Running server on: {}", listener.local_addr().unwrap());
    //run_server(listener, &conn);
    run_server(listener, &conn);
    println!("\nExiting server...");
}

pub fn run_server(listener: TcpListener, conn: &Connection) {
    let mut counter = 0;

    for stream in listener.incoming() { // seems like it gets stuck here on accept (in tcp-lib assembly code)
        if PRINTING {println!("f")};
        //let stream = stream.unwrap();
        let stream = stream.unwrap();

        counter += 1;
        if PRINTING {println!("Connection established! Counter {}", counter)};
        handle_connection(stream, counter, conn);
        println!("g");  /* Gets stuck here when response contains an image 
                         * (not just an image anymore, might have to do with failed connections) */
        //stream.flush().unwrap();
    }
}

/* 
pub fn run_server_new(listener: TcpListener, conn: &Connection) {
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    let mut counter = 0;

    let conn = init_database(true);

    while true {
        println!("here it comes!");
        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {addr:?}");
                counter += 1;
                if PRINTING {println!("Connection established! Counter {}", counter)};

                thread::spawn(move || {
                    handle_connection_multithreaded(_socket, counter, &conn);
                });
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                println!("Would block error: {e:?}");
                listener.set_nonblocking(false).expect("Cannot set non-blocking");
                continue;
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }

    

    /*
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                counter += 1;
                if PRINTING {println!("Connection established! Counter {}", counter)};
                handle_connection(stream, counter, &conn);
            }
            Err(error) => {
                println!("Connection failed:\n{}", error);
            }
        }
    } */

    /*
    while true {
        let stream_result = listener.accept();
        let stream = stream_result.unwrap();

        /*
        let (stream, socket_address) = match stream_result {
            Ok((stream, socket_address)) => {
                counter += 1;
                if PRINTING {println!("Connection established! Counter {}", counter)};
                (stream, socket_address)
                //handle_connection(stream, counter, &conn);
            },
            Err(error) => {
                
            },
        }; */
    }*/
}*/

#[allow(unused)]
fn handle_connection(mut stream: TcpStream, mut counter: i32, conn: &Connection) {
    println!("Running handle_connection...");

    // 1. Get request and convert to string
    let request = match server::get_request_string(&mut stream) {
        Ok(a) => a,
        Err(e) => {
            println!("Found error: {e}");
            return
        },
    };
    
    if(request.contains("Client:")) {
        println!("Received message: {}", request);
        let response = format!("Latest order data:\n{:?}", 
        database::table::get_query_iterator(conn, "SELECT * FROM [order] ORDER BY id DESC LIMIT 1;"));

        //stream.write("Server: Hello from server!".as_bytes()).unwrap(); // send response to tcp client
        stream.write(response.as_bytes()).unwrap(); // send response to tcp client
        stream.flush().unwrap();
        
        return;
    }

    let request_line = RequestLine::new(&request);

    // 2. Response creation
    let mut response = String::new();

    // 3. determine the type of http request and handle it to generate a suitable response
    response = handle_request(&request, conn);
    
    // TODO: add contains_resource block to request handler
    if contains_resource(&request_line.to_string()) {
        let mut file_content = Vec::new();
        //let path = Path::new("pogoPartyHatters.png");
        let file_name = "pogoPartyHatters.png";

        let mut file = File::open(&file_name).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

        //println!("File content:\n{:?}", file_content);
        println!("aaa");

        let mut response2 = response.to_string().into_bytes();
        println!("bbb");
        response2.extend(file_content);

        println!("ccc");

        //stream.write_all(&response2).unwrap(); // send http response to client
        stream.write(&response2).unwrap(); // send http response to client
        println!("ddd");
    } else {
        stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    }

    //stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    //stream.flush().unwrap();
    println!("eee");
    stream.take_error().expect("No error was expected...");
    //drop(response)
    //stream.shutdown(std::net::Shutdown::Both);
}

/* 
#[allow(unused)]
fn handle_connection_multithreaded(mut stream: TcpStream, mut counter: i32, conn: &Connection) {
    // check if request is empty:

    // 1. Get request and convert to string
    let request: String = server::get_request_string(&mut stream);
    if request.is_empty() || request.len() == 1 {
        println!("Empty request!");
        return;
    }
    let request_line = RequestLine::new(&request);

    // 2. Response creation
    let mut response = String::new();

    // 3. determine the type of http request and handle it to generate a suitable response
    response = handle_request(&request, &conn);
    
    // TODO: add contains_resource block to request handler
    if contains_resource(&request_line.to_string()) {
        let mut file_content = Vec::new();
        //let path = Path::new("pogoPartyHatters.png");
        let file_name = "pogoPartyHatters.png";

        let mut file = File::open(&file_name).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

        //println!("File content:\n{:?}", file_content);
        println!("aaa");

        let mut response2 = response.to_string().into_bytes();
        println!("bbb");
        response2.extend(file_content);

        println!("ccc");

        //stream.write_all(&response2).unwrap(); // send http response to client
        stream.write(&response2).unwrap(); // send http response to client
        println!("ddd");
    } else {
        stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    }

    //stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    //stream.flush().unwrap();
    println!("eee");
    stream.take_error().expect("No error was expected...");
    //drop(response)
    //stream.shutdown(std::net::Shutdown::Both);
}*/