#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rusqlite::{Connection, params, Result, Row};
use server::database::table::{print_rows_from_query, parse_query_to_json};

use crate::{request_line::RequestLine, database::table::get_query_iterator};

pub fn handle_fetch_request(request: &String, conn: &Connection) -> String {
    println!("Running fetch handler!");
    //crate::order_system::order_system::OrderSystem::hello_from_order_system();
    //order_system::order_system::OrderSystem::hello_from_order_system();

    let request_line = RequestLine::new(&request);
    let request_uri = &request_line.uri;
    let mut data = String::new();


    println!("Request line received:\n{}", request_line.to_string());
    println!("Request uri received:\n{}", request_uri);

    if request_uri.contains("api/v1/products") {
        //format!("{:?}", get_query_iterator(conn, "SELECT * FROM product;"));
        //println!("{:?}", get_query_iterator(conn, "SELECT * FROM product;"));
        let query = "SELECT * FROM product;";
        print_rows_from_query(conn, query).unwrap();

        let json_string = parse_query_to_json(conn, query);
        println!("json string:\n{}", json_string);
        data = json_string;   
    } else if request_uri.contains("ORDSYS/1.0") {
        
    }
    
    println!("Exiting fetch handler!");
    return data;
}