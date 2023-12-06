//use server::get_request_line;
//use server::get_request_line;
//use crate::database::table::{print_rows_from_query, parse_query_to_json}; //<--- funkar i fetch_han
//use crate::{request_line::RequestLine, database::table::get_query_iterator}; // <--- funkar inte heller
//use crate::request_line::*;

use rusqlite::Connection;
#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::{json, Value, Result};
use crate::database::order::Order;
#[allow(unused)]
use crate::order_system::order_system_testing::generate_order_request;

pub trait OrderSystemApiConstants {
    const API_IDENTIFIER: &'static str = "ORDSYS";
    const VERSION_1_0: &'static str = "1.0";
    
    const METHOD_PROCESS: &'static str = "PROCESS";
    const METHOD_REPORT: &'static str = "REPORT";

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

pub trait OrderSystemResponseApi {
    const RESPONSE_LINE_METHOD_UNKNOWN: &'static str = "ORDSYS/1.0 ERROR_METHOD_UNKNOWN";
    
    const RESPONSE_LINE_PROCESS_INCOMING: &'static str = "ORDSYS/1.0 INCOMING";
    const RESPONSE_LINE_PROCESS_NOT_READY: &'static str = "ORDSYS/1.0 NOT_READY";
    
    const RESPONSE_LINE_REPORT_OK: &'static str = "ORDSYS/1.0 OK";
    const RESPONSE_LINE_REPORT_FAIL: &'static str = "ORDSYS/1.0 FAIL";    
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

impl OrderSystemResponseApi for OrderSystemResponse {}

pub const CONSTANT_FROM_ORDER_SYSTEM: &str = "a";

pub fn hello_from_order_system() {
    println!("Hello from OrderSystem!");
}

#[allow(unused)]
impl OrderSystem {
    pub fn hello() {
        println!("Hello from OrderSystem!");
        println!("The constant: {}", CONSTANT_FROM_ORDER_SYSTEM);
    }    

    pub fn handle_request(request: &str, conn: &Connection) -> String {
        println!("Running handle_request() in Order System...\n");
        println!("Request received:\n{}\n", request);

        // 1. Create request line
        //let request_line = RequestLine::new(&request);
        let mut response = String::new();
        let request_line = request.lines().next().unwrap();
        println!("Request line received:\n{}\n", request_line);

        // 2. Check method
        let request_method = (&request_line).split(" ").next().unwrap();        
        println!("Request method received:\n{}\n", request_method);

        match request_method {
            "PROCESS" => {
                println!("Incoming PROCESS request!");
                response = Self::handle_process_request(request, conn);
            },            
            "REPORT" => {
                println!("Incoming REPORT request!");
                response = Self::handle_report_request(request, conn);
            },
            unknown_method => {
                println!("Received an unknown method: {}", unknown_method);
                response = OrderSystemResponse::RESPONSE_LINE_METHOD_UNKNOWN.to_string();
            }
        }
        
        println!("Sending following response to Order System:\n{}", response);

        response
    }

    pub fn handle_process_request(request: &str, conn: &Connection) -> String {        
        println!("Running handle_process_request()...");
        // 1. Create response
        let mut response = String::new();

        // 2. Check body
        let body_line = request.lines().last().unwrap().replace("\0", "").to_string();
        println!("Request body received:\n{}\n", body_line);

        // 3. Get oldest order that is ready for processing
        let order_id_result = Order::get_oldest_ready_order_id(conn);
        let order_id = match order_id_result {
            Ok(id) => {
                println!("Order ready for processing exists with id: {id}!");
                // 3.1 Update response
                response = OrderSystemResponse::RESPONSE_LINE_PROCESS_INCOMING.to_string();
                response.push_str(&format!("\n{}", Order::create_order_response_full(conn, id)));
                
                // 3.2 Update order status to PROCESSING
                println!("Order before update:");
                crate::database::table::print_rows_from_query(conn, &format!("select * from [order] WHERE id = {id};"));
                Order::update_order_status_to_processing(conn, &id);
                println!("Order after update:");
                crate::database::table::print_rows_from_query(conn, &format!("select * from [order] WHERE id = {id};"));

                //response = Order::create_order_response_full(conn, id);
                id
            }
            Err(e) =>  {
                println!("Problem getting order id of oldest ready order: {:?}", e);
                response = OrderSystemResponse::RESPONSE_LINE_PROCESS_NOT_READY.to_string();
                -1
            }
        };

        println!("Exiting handle_process_request()...");
        response
    }

    pub fn handle_report_request(request: &str, conn: &Connection) -> String {        
        println!("Running handle_report_request()...");
        // 1. Create response
        let mut response = String::new();

        // 2. Check status
        let status = Self::get_report_status(request);
        println!("Received status: {status}");

        match status.as_str() {
            OrderSystemApi::STATUS_OK => {
                println!("Order process was successful!");
                // 1. Update response
                response = OrderSystemResponse::RESPONSE_LINE_REPORT_OK.to_string();

                // 2. Update order status 

                
            },

            OrderSystemApi::STATUS_FAIL => {
                println!("Order process not successful!");
                // 1. Update response
                response = OrderSystemResponse::RESPONSE_LINE_REPORT_FAIL.to_string();

                // 2. Update order status 

                // 3. Update order positions

                // 4. Update product amounts 
            },

            _ => {

            },

        }


        // 3. Check body

        println!("Exiting handle_report_request()...");
        response
    }

    pub fn get_order_id_from_json(values: &serde_json::Value) -> i64 {
        serde_json::from_value(values["order_id"].to_owned()).unwrap()
    }

    pub fn get_report_status(request: &str) -> String {
        String::from(request.split("\n").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[1].trim())
    }
}




