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
#[allow(unused)]
use server::{database::order, order_system::order_system::*};
//use server::database::database::hello_from_database;

#[allow(unused)]
use crate::{init::*, request_handler::handle_request, order_system::order_system::{OrderSystemRequest, OrderSystemRequestApi}};
use crate::get_handler::*;
#[allow(unused)]
use crate::post_handler::*;
use crate::request_line::*;
use crate::database::database::*;
//use crate::order_system::order_system::*;

mod init;
mod request_handler;
mod get_handler;
mod post_handler;
mod request_line;
mod database;
mod fetch_handler;
pub mod order_system;
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
    let conn = init_database(true);

    //test_bufReader();
    order_system::order_system_testing::test_order_system(&conn, &1);

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