#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader}, path::Path,
};

use std::fs::File;

use rusqlite::Connection;
#[allow(unused)]
use server::get_request_line;

use crate::{
    request_line::RequestLine, 
    database::{product::Product, table::get_query_iterator}, 
    fetch_handler::fetch_handler::handle_fetch_request
};

pub fn handle_get_request_new(request: &String, conn: &Connection) -> String {
    println!("---------------------------------------------------------");
    println!("Hello from handle_get_request_new()!\n");
    println!("Request received:\n{}", request);
    let mut response = String::new();
    let mut resource = String::new();

    // 1. Get request line
    let request_line = RequestLine::new(&request);
    //let request_line = get_request_line(request);
    println!("Request line received from struct: {}", request_line.to_string());
    println!("Request method from struct: {}", request_line.method);

    // 2. Get URI
    println!("Request URI from struct: {}", request_line.uri);

    // 3. Get format/file type
    println!("Request URI file type from struct: {}", request_line.uri_file_type);

    // 4. Check if file exists in server
    if Path::new(&request_line.get_request_uri_without_first_slash()).exists() {
        println!("File path for uri {} exists!", &request_line.get_request_uri_without_first_slash());
    } else {
        println!("File path for uri {} does not exist!", request_line.uri);
        // return error response
    }

    let mut image_request = false;
    let mut data_request = false;
    let mut data = String::new();

    let (status_line, filename) = if Path::new(&request_line.get_request_uri_without_first_slash()).exists() {
        // 5. Check if file is text or binary
        if is_binary(&request_line.to_string()) {
            println!("{} is a binary file",  request_line.get_request_uri_without_first_slash());
            image_request = true;
        } else {
            println!("{} is a text file",  request_line.get_request_uri_without_first_slash());
        }
        ("HTTP/1.1 200 OK", request_line.get_request_uri_without_first_slash())
    } else {
        if &request_line.to_string() == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if (&request_line).to_string().contains("api/v") {
            data_request = true;
            data = handle_fetch_request(request, conn);

            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if (&request_line).to_string().contains("something") {
            data_request = true;
            data = String::from("hankey");
            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if  (&request_line).to_string().contains("latest-product") {
            data_request = true;
            //data = Product::get_latest_product(conn);
            //data = String::from_utf8_lossy(get_query_iterator(conn, ""));
            let data_vector = get_query_iterator(conn, "SELECT * FROM product ORDER BY id DESC LIMIT 1;");
            data = format!("{:?}", data_vector);

            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if  (&request_line).to_string().contains("latest-order") {
            data_request = true;
            data = format!("{:?}", get_query_iterator(conn, "SELECT * FROM [order] ORDER BY id DESC LIMIT 1;"));

            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if  (&request_line).to_string().contains("oldest-product") {
            data_request = true;
            data = Product::get_oldest_product(conn);
            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else if  (&request_line).to_string().contains("cheapest-product") {
            data_request = true;
            data = Product::get_cheapest_product(conn);
            ("HTTP/1.1 200 OK", String::from("index.html"))
        } else {
            ("HTTP/1.1 404 NOT FOUND", String::from("error.html"))
        }
    };
    
    println!("Getting file contents...");
    let mut contents = String::new();
    
    let mut length = 0;

    if data_request {
        contents = data;

        length = contents.len();
        let data_response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        println!("Data respnse: \n{}", data_response);
        return data_response
    }

    let mut file_content = Vec::new();

    if !image_request {
        //contents = fs::read_to_string(filename).unwrap();
        contents = match fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("e");
                fs::read_to_string("error.html").unwrap()
            }
        };
        length = contents.len();
    } else {
        let file_name = &request_line.get_request_uri_without_first_slash();
        resource = file_name.to_string();
        println!("File name {file_name}");

        let mut file = File::open(&file_name).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");
        length = file_content.len();        
    }
    println!("Got file contents...");
    response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    if image_request {
        response.push_str(&resource);
    }

    //println!("Response from helper:\n{}", response);
    println!("---------------------------------------------------------");
    response
}

// placeholder, need to check if resource exists in server first
pub fn contains_resource(request_header: &str) -> bool {
    request_header.contains(".png") || request_header.contains(".jpg") || request_header.contains(".jpeg")
}

pub fn is_text(request_line: &str) -> bool {
    request_line.contains(".txt") || request_line.contains(".css") || request_line.contains(".js")
}

pub fn is_binary(request_line: &str) -> bool {
    //!is_text(request_line)
    request_line.contains(".png") || request_line.contains(".jpg") || request_line.contains(".jpeg")
}


