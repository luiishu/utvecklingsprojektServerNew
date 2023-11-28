#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
};

use rusqlite::Connection;

use crate::{request_line::RequestLine, database::{database::{register_user, print_rows_from_user}, product::Product, table::Table}};

#[allow(unused)]
pub fn handle_post_request_new(request: &String, conn: &Connection) -> String {
    let mut response = String::new();
    let request_line = RequestLine::new(&request);

    println!("Hello from handle_post_request_new!");
    //println!("Request received:\n{}", request);
    //println!("Request line received:\n{}", request_line.to_string());

    let body = server::get_body_line2(&request.to_string());
    //assert_eq!(body2, "fname=John&lname=Johnson");

    //println!("b");
    println!("Request body: {}", body);
    let v: Vec<Vec<&str>> = server::parse_body(&body);
    println!("Request body vector 3: {:?}", v);
    println!("[table, table name]: [{}, {}]", v[0][0], v[0][1]);
    let table = v[0][1];
    if table == "user" {
        register_user(&conn, v);
        print_rows_from_user(&conn);
    } else if table == "product" {
        Product::insert(&conn, v);
        Product::print_rows(&conn);
            
    } else {
        server::insert_fname_and_lname_into_person(&conn, v[0][1], v[1][1]).unwrap();
        server::print_rows_from_person(&conn).unwrap();
    }

    //println!("c");


    let (status_line, filename) = ("HTTP/1.1 204 No Content", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    response
}

#[allow(unused)]
pub fn handle_post_request(request_line: &String) -> String {
    println!("---------------------------------------------------------");
    println!("Hello from handle_post_request_new!");
    println!("Request line received: {}", request_line);

    let (status_line, filename) = ("HTTP/1.1 204 No Content", "index.html");

    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //println!("response from helper:\n {}", response);
    println!("---------------------------------------------------------");
    response
}
