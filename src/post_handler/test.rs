use std::fs;

use rusqlite::Connection;

use crate::{request_handler, database::{order::Order, table::Table as _, order_item::OrderItem, user::User, product::Product}};

pub struct PostTester;

pub fn test(conn: &Connection) {
    println!("Running post handler test...");
    test_orders(conn);
    //test_users(conn);
    
    
    println!("Exiting post handler test...");
}

fn test_orders(conn: &Connection) {
    let request_line = String::from("POST /web_server/api/v1/orders HTTP/1.1");
    print!("Printing all orders BEFORE test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    Order::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("Printing all order items BEFORE test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    OrderItem::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("Printing all products BEFORE test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    Product::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");

    test_orders2(conn);


    print!("Printing all orders AFTER test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    Order::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("Printing all order items AFTER test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    OrderItem::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("Printing all products AFTER test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    Product::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
}

pub fn test_orders1(conn: &Connection) {
    let request_line = String::from("POST /web_server/api/v1/orders HTTP/1.1");

    let body = fs::read_to_string("json/order_posting.json").expect("Unable to open file");    
    let mut request = format!("{request_line}\n{body}");

    println!("\nSending test request to request handler!\n");
    let response = request_handler::handle_request(&request, conn);
    println!("Request sent:\n{request}");
    println!("Response received:\n{response}");
}

pub fn test_orders2(conn: &Connection) {
    let request_line = String::from("POST /web_server/api/v1/orders HTTP/1.1");

    let body = fs::read_to_string("json/order_posting2.json").expect("Unable to open file");    
    let mut request = format!("{request_line}\n{body}");

    println!("\nSending test request to request handler!\n");
    let response = request_handler::handle_request(&request, conn);
    println!("Request sent:\n{request}");
    println!("Response received:\n{response}");
}

fn test_users(conn: &Connection) {
    println!("Printing all users BEFORE test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    User::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");

    let request_line = String::from("POST /web_server/api/v1/users HTTP/1.1");
    let body = r#"{"username":"hankey420","password":"abc123"}"#;
    let mut request = format!("{request_line}\n{body}");

    println!("Request from post handler test:\n{request}");
    println!("\nSending test request to request handler!\n");
    request_handler::handle_request(&request, conn);

    println!("Printing all users AFTER test:");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    User::print_rows(conn).unwrap();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
}



