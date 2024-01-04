#![allow(unused)]
// put main logic from handle_connection here
// goal: get method of the incoming request and instruct a specific handler to handle it

use std::ops::Add;

use rusqlite::Connection;

use crate::{request_line::RequestLine, get_handler::handle_get_request_new, post_handler::post_handler::*};

pub fn handle_request(request: &str, conn: &Connection) -> String {
    let mut response = String::new();
    let request_line = RequestLine::new(&request);

    let request_method = &request_line.method;

    if request_method == "GET" {
        println!("Incoming GET-request from client!");
        response = handle_get_request_new(&request.to_string(), conn);

    } else if request_method == "POST" {
        println!("Incoming POST-request from client!");
        response = PostHandler::handle_post_request(&request.to_string(), &conn);

    } else {
        println!("Unknown request from client!");
        response = handle_get_request_new(&request.to_string(), conn);
    }

    if!response.is_empty() {
        let response_line = response.split("\n").next().unwrap();
        let new_response = response.replace(response_line, &format!("{response_line}\nConnection: close"));
        response = new_response;
        //response.push_str("FIN");
    }

    //println!("Response from request handler:\n{}", response);
    
    response
}