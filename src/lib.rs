pub mod database;
pub mod order_system;


#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

#[allow(unused)]
use rusqlite::{Connection, params, Result};

const RUST_PORT: u16 = 7878;

pub fn get_body_line2(http_request: &str) -> String {
    let body_line = http_request.lines().last().unwrap().replace("\0", "").to_string();
    body_line
}

pub fn init_server_on_localhost() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}

pub fn init_server_on_lan() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 178));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), ip_v4);

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}

pub fn get_request_string(mut stream: & TcpStream) -> Result<String, &'static str> {
    let mut buffer = [0u8; 2048]; // store stream bytes
    //let mut vector: Vec<u8> = Vec::new();
    //let mut buffer = [0u8; 4096]; // store stream bytes

    let bytes_read = stream.read(&mut buffer).unwrap(); // store stream bytes in buffer
    //let bytes_read = stream.read_to_end(&mut vector).unwrap(); // store stream bytes in buffer
    if bytes_read <= 0 {
        return Err("Empty request error. Abort current connection.");
    }
    println!("Bytes read from from stream: {}", bytes_read);

    let request = String::from_utf8_lossy(&buffer[..]).to_string(); /* convert buffer to string
                                                                             *  use to_string() to convert
                                                                             * to  String */
    Ok(request)
}

pub fn get_request_string_old(mut stream: & TcpStream) -> String {
    let mut buffer = [0u8; 2048]; // store stream bytes
    //let mut vector: Vec<u8> = Vec::new();
    //let mut buffer = [0u8; 4096]; // store stream bytes

    let bytes_read = stream.read(&mut buffer).unwrap(); // store stream bytes in buffer
    //let bytes_read = stream.read_to_end(&mut vector).unwrap(); // store stream bytes in buffer
    println!("Bytes read from from stream: {}", bytes_read);

    let request = String::from_utf8_lossy(&buffer[..]).to_string(); /* convert buffer to string
                                                                             *  use to_string() to convert
                                                                             * to  String */
    request
}

pub fn get_request_line(http_request: &str) -> String {
    http_request.lines().next().unwrap().to_string()
}

pub fn get_body_line(http_request: &String) -> String {
    http_request.lines().last().unwrap().to_string()
}

pub fn parse_body<'a>(body: &str) -> Vec<Vec<&str>> {
    body.split("&").map(|s| s.split("=").collect()).collect()
}



pub fn insert_fname_and_lname_into_person(conn: &Connection, fname: &str, lname: &str) -> Result<()> {
    println!("Trying to insert {} {} into Person", fname, lname);
    conn.execute(
        "INSERT INTO person (fname, lname) VALUES (?1, ?2)",
        (fname, lname),
    )?;

    Ok(())
}

pub fn print_rows_from_person(conn: &Connection) -> Result<()> {
    println!("Running print_rows_from_person()...");

    let mut stmt = conn.prepare("SELECT * FROM person")?;
    //println!("Statement created!");

    let name_iter = stmt.query_map([], |row| {
        //println!("id");
        let id: i64 = row.get(0)?;
        //println!("fname!");
        let fname: String = row.get(1)?;
        //println!("lname!");
        let lname: String = row.get(2)?;

        let cols = vec![id.to_string(), fname, lname];

        Ok(cols)
    })?;

    //println!("Iterator created!");

    for name in name_iter {
        println!("Found person [id, fname, lname]: {:?}", name.unwrap());
    }

    println!("Exiting print_rows_from_person()...");

    Ok(())
}
