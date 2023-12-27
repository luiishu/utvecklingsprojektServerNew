#![allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process, str::from_utf8,
};

use std::{fs::File, thread::{self, Thread}};
use rand::Rng;

use rusqlite::{Connection, params, Result};

use server::{database::order, order_system::order_system::*};
//use server::database::database::hello_from_database;

use crate::{init::*, request_handler::handle_request, order_system::order_system::{OrderSystemRequest, OrderSystemRequestApi}};
use crate::get_handler::*;
use crate::post_handler::*;
use crate::request_line::*;
use crate::database::database::*;
//use crate::order_system::order_system::*;

mod database;
mod init;
mod fetch_handler;
mod get_handler;
mod order_system;
mod post_handler;
mod response;
mod request_handler;
mod request_line;
//mod database/hello_from_database;
//static mut counter: i32 = 0;

const LAN: bool = false;
const PRINTING: bool = true;

const LOCALHOST_IP_V4: &str = "127.0.0.1";
const SERVER_IP_V4: &str = "192.168.1.178";

const RUST_PORT: u16 = 7878;

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

fn handle_connection(mut stream: TcpStream, mut counter: i32, conn: &Connection) {
    println!("Running handle_connection...");

    // 1. Get request and convert to string
    let request = match server::get_request_string(&mut stream) {
        Ok(request) => request,
        Err(e) => {
            println!("Found error: {e}");
            return
        },
    };
    
    // 1.2 Check if request is from Order System and handle it and respond if true 
    if(!request.contains("HTTP")) {
        let response = OrderSystem::handle_request(&request, conn);

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
        let mut file_content: Vec<u8> = Vec::new();
        let file_name = response.split("\n").last().unwrap();
        let mut response = response.replace(file_name, "");

        //println!("Response:\n{response}");
        //println!("Received resource in connection hanlder: {file_name}");

        let mut file = File::open(&file_name).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

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
        return
    } else {
        stream.write_all(response.as_bytes()).unwrap(); // send http response to client
    }

    println!("eee");
    stream.take_error().expect("No error was expected...");
    //drop(response)
    //stream.shutdown(std::net::Shutdown::Both);
}