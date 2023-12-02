//use server::get_request_line;
//use server::get_request_line;
//use server::database::table::{print_rows_from_query, parse_query_to_json}; //<--- funkar i fetch_han
//use crate::{request_line::RequestLine, database::table::get_query_iterator}; // <--- funkar inte heller

#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::{json, Value, Result};

pub trait OrderSystemApiConstants {
    const API_IDENTIFIER: &'static str = "ORDSYS";
    const VERSION_1_0: &'static str = "1.0";
    
    const METHOD_PROCESS: &'static str = "PROCESS";
    const METHOD_REPORT: &'static str = "PROCESS";

    const STATUS_OK: &'static str = "OK";
    const STATUS_FAIL: &'static str = "FAIL";
}

pub trait OrderSystemRequestApi {
    const REQUEST_PROCESS_FULL: &'static str = "PROCESS orders/latest ORDSYS/1.0\n{”status”: ”processing”}";
    const REQUEST_LINE_PROCESS: &'static str = "PROCESS orders/latest ORDSYS/1.0";
    

    const REQUEST_LINE_REPORT: &'static str = "REPORT orders/id ORDSYS/1.0";
    
    const REQUEST_REPORT_OK_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: OK";
    const REQUEST_LINE_REPORT_OK: &'static str = "REPORT orders/id ORDSYS/1.0";

    const REQUEST_REPORT_FAIL_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: FAIL";    
}

pub struct OrderSystem;

pub struct OrderSystemApi;

pub struct OrderSystemRequest;
pub struct OrderSystemResponse;

impl OrderSystemRequestApi for OrderSystemRequest {}
//impl OrderSystemApi for OrderSystemRequest {}
impl OrderSystemApiConstants for OrderSystemApi {}

impl OrderSystemRequest {
    pub fn build_request(method: &str, resource: &str, version: &str) -> String {
        if method.is_empty() || resource.is_empty() || version.is_empty() {
            return String::from("Empty input error");
        }

        let request = format!("{method} {resource} {}/{version}", OrderSystemApi::API_IDENTIFIER);

        request
    }
}

pub const CONSTANT_FROM_ORDER_SYSTEM: &str = "a";

pub fn hello_from_order_system() {
    println!("Hello from OrderSystem!");
}

impl OrderSystem {
    pub fn hello() {
        println!("Hello from OrderSystem!");
        println!("The constant: {}", CONSTANT_FROM_ORDER_SYSTEM);
    }    

    pub fn handle_proccess(request: &str) -> String{
        println!("Request received:\n{}", request);

        // 1. Create request line
        //let request_line = RequestLine::new(&request);
        //let request_line = get_request_line(request);


        // 2. Check method


        //let response = format!("Latest order data:\n{:?}", 
        //database::table::get_query_iterator(conn, "SELECT * FROM [order] WHERE status = \"READY\" ORDER BY id ASC LIMIT 1;"));
        let response = format!("ACK from server!");

        response
    }

    pub fn get_order_id_from_json(values: &serde_json::Value) -> i64 {
        serde_json::from_value(values["order_id"].to_owned()).unwrap()
    }
}


