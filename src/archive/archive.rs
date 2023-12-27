#![allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

use rand::Rng;

fn init_database() {
    /*
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    client.simple_query("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
    ").unwrap();
    */
}

fn handle_connection_test(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes

    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer

    let request = String::from_utf8_lossy(&buffer[..]); /* convert buffer to string
                                                                 *  use to_string() to convert to
                                                                 *  String*/

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_test2(mut stream: TcpStream, mut counter: i32) {
    let mut buf_reader = BufReader::new(&mut stream);

    //let http_request = String::new();
/*
    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();*/

    //if PRINTING {println!("Request:\n{}", http_request);} // need to examine borrow errors with buf_reader

    //if PRINTING {println!("Request:\n{:#?}", http_request);} // need to examine borrow errors with buf_reader

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    //stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_new(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes
    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer
    let request = String::from_utf8_lossy(&buffer[..]); // convert buffer to string
    //let mut response: String = String::new();

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
        //let response = handle_get_request(request);
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);
    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");





    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
    stream.write_all(response.as_bytes()).unwrap();
}

fn test_bufReader() {
    // 1. Printing text file contents
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    println!("\n1. Reading file_contents:\n");
    for line in buf_reader.lines() {
        println!("{}", line.unwrap());
    }

    // 2. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let file_contents = io::read_to_string(buf_reader).unwrap();

    println!("\n2.1 Reading file_contents:\n{:?}", file_contents);
    println!("\n2.2 Reading file_contents:\n{}", file_contents);

    // 3. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut file_contents: String = String::new();

    for line in buf_reader.by_ref().lines() {
        //buf_reader.read_line(&mut file_contents);
        file_contents.push_str(line.unwrap().as_str());
        file_contents.push_str("\n");
    }

    println!("\n3.1 Reading file_contents:\n{:?}", file_contents); // printing raw string
    println!("\n3.2 Reading file_contents:\n{}", file_contents); // pretty printing with newlines
}

fn test_array_buffer() {

}

/* **************************************************************************************************
 * Old get handler function
 *****************************************************************************************************/
pub fn handle_get_request_new(request: &String, conn: &Connection) -> String {
    println!("---------------------------------------------------------");
    println!("Hello from handle_get_request_new()!\n");
    println!("Request received:\n{}", request);
    let mut response = String::new();

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
        contents = fs::read_to_string(filename).unwrap();
        length = contents.len();
    } else {
        //let mut file_content = Vec::new();
        //let path = Path::new("pogoPartyHatters.png");
        //let file_name = "pogoPartyHatters.png";
        let file_name = &request_line.get_request_uri_without_first_slash();
        println!("File name {file_name}");

        let mut file = File::open(&file_name).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");
        length = file_content.len();        
    }
    println!("Got file contents...");
    //length = contents.len();
    response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    if image_request {
        //response.push_str((&file_content));
        let mut response2 = response.to_string().into_bytes();
        response2.extend(file_content);
        //return String::from_utf8(response2).unwrap()
        //return response2.iter().map(|&c| c as char).collect::<String>();
        //return base64::encode(&response2)
    }

    println!("Response from helper:\n{}", response);
    println!("---------------------------------------------------------");
    response
}

/* **************************************************************************************************
 * Old post handler functions
 ******************************************************************************************************/
 
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
    println!("Request body vector: {:?}", v);
    //println!("[table, table name]: [{}, {}]", v[0][0], v[0][1]);
    //let table = v[0][1];
    let table = "";
    if table == "user" {
        //register_user(&conn, v);
        //print_rows_from_user(&conn);
    } else if table == "product" {
        //Product::insert(&conn, v);
        //Product::print_rows(&conn);
            
    } else {
        //server::insert_fname_and_lname_into_person(&conn, v[0][1], v[1][1]).unwrap();
        //server::print_rows_from_person(&conn).unwrap();
    }

    //println!("c");


    //let (status_line, filename) = ("HTTP/1.1 204 No Content", "index.html");
    //let contents = fs::read_to_string("index.html").unwrap();
    //let length = contents.len();
    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    //let contents = fs::read_to_string("index.html").unwrap();
    //let length = contents.len();

    //response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    response = String::from("HTTP/1.1 200 OK");

    response
}

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
