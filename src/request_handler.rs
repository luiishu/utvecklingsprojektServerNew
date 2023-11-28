// put main logic from handle_connection here
// goal: get method of the incoming request and instruct a specific handler to handle it

use rusqlite::Connection;

use crate::{request_line::RequestLine, get_handler::handle_get_request_new, post_handler::handle_post_request_new};

pub fn handle_request(request: &str, conn: &Connection) -> String {
    #[allow(unused)]
    let mut response = String::new();
    let request_line = RequestLine::new(&request);

    let request_method = &request_line.method;

    if request_method == "GET" {
        println!("Incoming GET-request from client!");
        response = handle_get_request_new(&request.to_string(), conn);

    } else if request_method == "POST" {
        println!("Incoming POST-request from client!");
        response = handle_post_request_new(&request.to_string(), &conn);

    } else {
        println!("Unknown request from client!");
        response = handle_get_request_new(&request.to_string(), conn);
    }

    //println!("{}", response);
    response
}