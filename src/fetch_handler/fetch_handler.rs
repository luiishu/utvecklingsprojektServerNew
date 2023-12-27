#![allow(unused)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::panic::catch_unwind;

use rusqlite::{Connection, params, Result, Row};
use server::database::table::{print_rows_from_query, parse_query_to_json};

use crate::{request_line::RequestLine, database::table::{get_query_iterator, self}};

fn is_last_resource(id: usize, resource: Vec<&str>) -> bool {
    (id + 1) == resource.len()
}

fn parse_fetch_resource_to_query(mut resource: String) -> String {
    if resource.contains('-') {
        //println!("Removing!");
        resource = resource.replace("-", "_");
    }

    let mut resource_chars = resource.chars();
    resource_chars.next_back();
    if resource == "addresses" {resource_chars.next_back();}
    
    let table_name = format!("[{}]", resource_chars.as_str());

    println!("Table name: {table_name}");

    let query = format!("SELECT * FROM {table_name};");
    query
}

pub fn handle_fetch_request(request: &String, conn: &Connection) -> String {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("Running fetch handler!");
    //crate::order_system::order_system::OrderSystem::hello_from_order_system();
    //order_system::order_system::OrderSystem::hello_from_order_system();

    // Get request line
    let request_line = RequestLine::new(&request);
    println!("Request line received:\n{}", request_line.to_string());

    // Get request uri
    let request_uri = &request_line.uri;
    println!("Request uri received:\n{}", request_uri);
    let mut request_uri_vector: Vec<&str> = request_uri.split("/").collect();
    request_uri_vector.remove(0);
    println!("Request uri vector:\n{:?}", request_uri_vector);
    
    // Create response
    let mut data = String::new(); // <-- response? 

    // Get location
    let mut current_id = 0;
    let location = *request_uri_vector.get(current_id).unwrap();
    current_id += 1;
    println!("Location: {location}");

    // 1. Get api name
    let api_name = *request_uri_vector.get(current_id).unwrap();
    println!("API name: {api_name}");
    current_id += 1;

    // 1.1 Check validity of api name

    // 1.2 Proceed if valid, otherwise return

    // 2. Get api version
    let api_version = *request_uri_vector.get(current_id).unwrap();
    println!("API version: {api_version}");
    current_id += 1;

    // 2.2 Check validity of api version

    // 2.2 Proceed if valid, otherwise return

    // 3. Get resource
    //let resource_id = 3;
    let resource = String::from(*request_uri_vector.get(current_id).unwrap());    
    println!("Resource: {resource}");
    

    // 3.1 Check validity of resource

    // 3.2 Proceed if valid, otherwise return

    // Check if resource is last resource
    println!("Resource \"{resource}\" is last resource: {}", is_last_resource(current_id, request_uri_vector));

    if true {
        let query = parse_fetch_resource_to_query(resource);

        let mut json_string = parse_query_to_json(conn, &query);
        //json_string = json_string.replace("\"rows\":", "\"data\":");
        println!("JSON string:\n{}", json_string);
        data = json_string;   
    }
    else if request_uri.contains("api/v1/products") {
        //format!("{:?}", get_query_iterator(conn, "SELECT * FROM product;"));
        //println!("{:?}", get_query_iterator(conn, "SELECT * FROM product;"));
        let query = "SELECT * FROM product;";
        print_rows_from_query(conn, query).unwrap();

        let json_string = parse_query_to_json(conn, query);
        println!("json string:\n{}", json_string);
        data = json_string;   
    } else if request_uri.contains("ORDSYS/1.0") {
        // should not be used here
    }
    
    println!("Exiting fetch handler!");
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    return data;
}