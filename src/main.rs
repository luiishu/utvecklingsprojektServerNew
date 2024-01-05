#![allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    process,
    str::from_utf8,
    time::{Duration, SystemTime},
};

use rand::Rng;
use std::{
    fs::File,
    thread::{self, Thread},
};

use rusqlite::{params, Connection, Result};

use server::{database::order, order_system::order_system::*};
//use server::database::database::hello_from_database;

use crate::database::database::*;
use crate::get_handler::*;
use crate::post_handler::*;
use crate::request_line::*;
use crate::{
    init::*,
    order_system::order_system::{OrderSystemRequest, OrderSystemRequestApi},
    request_handler::handle_request,
    response::response::{HttpResponseCode, HttpResponseCodes, ResponseLine},
};
//use crate::order_system::order_system::*;

mod database;
mod fetch_handler;
mod get_handler;
mod init;
mod order_system;
mod post_handler;
mod request_handler;
mod request_line;
mod response;
//mod database/hello_from_database;
//static mut counter: i32 = 0;

const LAN: bool = true;
const PRINTING: bool = true;

const LOCALHOST_IP_V4: &str = "127.0.0.1";
const SERVER_IP_V4: &str = "192.168.88.221"; //

const RUST_PORT: u16 = 7878;

pub const NONBLOCKING: bool = true;
pub const BLOCKING: bool = !NONBLOCKING;

pub const SLEEP_DURATION_IN_MILLISECONDS: u64 = 1000;
pub const SLEEP_DURATION: Duration = Duration::from_millis(SLEEP_DURATION_IN_MILLISECONDS);

//static conn: Connection = Connection::open_in_memory().unwrap();

fn main() {
    //hello();
    //hello_from_database();

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
    let conn = database::init::init_database(true);
    //database::test::database_testing::test(&conn).unwrap();

    //test_bufReader();
    //order_system::order_system_testing::test_order_system(&conn, &1);
    //fetch_handler::test::test();
    post_handler::test::test(&conn);

    println!();

    println!("Running server on: {}", listener.local_addr().unwrap());
    //run_server(listener, &conn);
    run_server(listener, &conn);
    println!("\nExiting server...");
}

pub fn run_server(listener: TcpListener, conn: &Connection) {
    let mut counter = 0;

    for stream in listener.incoming() {
        // seems like it gets stuck here on accept (in tcp-lib assembly code)
        //if PRINTING {println!("f")};
        match stream {
            Ok(stream) => {
                counter += 1;
                if PRINTING {
                    println!("Connection established! Counter {}", counter)
                };
                handle_connection(stream, counter, conn);
                println!("g");
                continue;
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //eprintln!("Expected: {e}");
                //println!("No connection received, so I sleep...");
                std::thread::sleep(SLEEP_DURATION);
                continue;
            }

            Err(e) => {
                eprintln!("Received a different error: {e}");
                continue;
            }
        }

        println!("What am I doing here?");
    }
}

fn handle_connection(mut stream: TcpStream, mut counter: i32, conn: &Connection) {
    println!("Running handle_connection...");

    // 1. Get request and convert to string
    let request = match server::get_request_string(&mut stream) {
        Ok(request) => request,
        Err(e) => {
            println!("Found error: {e}");
            return;
        }
    };

    // 1.2 Check if request is from Order System and handle it and respond if true
    if (!request.contains("HTTP")) {
        let response = OrderSystem::handle_request(&request, conn);

        //stream.write(response.as_bytes()).unwrap(); // send response to tcp client
        match stream.write(response.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to write response to Order System...");
                eprintln!("{e}");
            }
        }

        match stream.flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
            }
        }

        return;
    }

    let request_line = RequestLine::new(&request);

    // 2. Response creation
    let mut response = String::new();

    // 3. determine the type of http request and handle it to generate a suitable response
    response = handle_request(&request, conn);

    // TODO: add contains_resource block to request handler
    if contains_resource(&request_line.to_string()) {
        let mut file_content: Vec<u8> = Vec::new();
        //let file_name = response.split("\n").last().unwrap();
        let file_name = response.split("\n").last();

        let file_name: &str = match file_name {
            Some(file_name) => file_name,
            None => {
                response = ResponseLine::get_response_line(HttpResponseCode::NOT_FOUND);
                stream.write(response.as_bytes()).unwrap();
                return;
            }
        };

        let mut response = response.replace(file_name, "");

        //println!("Response:\n{response}");
        //println!("Received resource in connection hanlder: {file_name}");

        //let mut file = File::open(&file_name).expect("Unable to open file");
        let mut file = match File::open(&file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{e}");
                response = ResponseLine::get_response_line(HttpResponseCode::INTERNAL_SERVER_ERROR);
                stream.write(response.as_bytes()).unwrap();
                return;
            }
        };

        //file.read_to_end(&mut file_content).expect("Unable to read");
        match file.read_to_end(&mut file_content) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                response = ResponseLine::get_response_line(HttpResponseCode::INTERNAL_SERVER_ERROR);
                stream.write(response.as_bytes()).unwrap();
                return;
            }
        }

        //println!("File content:\n{:?}", file_content);
        //println!("aaa");
        //println!("Response: {response}");

        let mut response2 = response.to_string().into_bytes();
        //println!("bbb");
        response2.extend(file_content);

        //println!("ccc");
        //println!("Response:\n{response}");

        stream.write(&response2).unwrap(); // send http response to client
                                           //println!("ddd");
        return;
    } else {
        stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    }

    println!("eee");
    //stream.take_error().expect("No error was expected...");
    match stream.take_error() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
        }
    }

    //drop(response)
    //stream.shutdown(std::net::Shutdown::Both);
}
