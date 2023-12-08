use std::num::ParseIntError;

use rusqlite::Connection;
#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::{json, Value};
use crate::database::{order::Order, order_position::OrderPosition, table::{get_query_iterator, print_rows_from_query, parse_query_to_json}};
#[allow(unused)]
use crate::order_system::order_system_testing::generate_order_request;

pub trait OrderSystemApiConstants {
    const API_IDENTIFIER: &'static str = "ORDSYS";
    const VERSION_1_0: &'static str = "1.0";
    
    const METHOD_GET: &'static str = "GET";
    const METHOD_PROCESS: &'static str = "PROCESS";
    const METHOD_REPORT: &'static str = "REPORT";

    const STATUS_OK: &'static str = "OK";
    const STATUS_FAIL: &'static str = "FAIL";
}

pub trait OrderSystemRequestApi {
    const REQUEST_PROCESS_FULL: &'static str = r#"PROCESS orders/oldest ORDSYS/1.0\n{"status": "processing"}"#;
    const REQUEST_LINE_PROCESS: &'static str = "PROCESS orders/oldest ORDSYS/1.0";

    const REQUEST_GET_ORDER_POSITIONS_FULL: &'static str = "GET order-positions ORDSYS/1.0";
    
    const REQUEST_LINE_REPORT: &'static str = "REPORT orders/id ORDSYS/1.0";
    
    const REQUEST_REPORT_OK_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: OK";
    const REQUEST_LINE_REPORT_OK: &'static str = "REPORT orders/id ORDSYS/1.0";

    const REQUEST_REPORT_FAIL_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: FAIL";
}

pub trait OrderSystemResponseApi {
    const RESPONSE_LINE_RESOURCE_UNKNOWN: &'static str = "ORDSYS/1.0 ERROR_RESOURCE_UNKNOWN";
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
            OrderSystemApi::METHOD_GET => {
                println!("Incoming GET request!");
                response = Self::handle_get_request(request, conn);
            },
            OrderSystemApi::METHOD_PROCESS => {
                println!("Incoming PROCESS request!");
                response = Self::handle_process_request(request, conn);
            },
            OrderSystemApi::METHOD_REPORT => {
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

    pub fn handle_get_request(request: &str, conn: &Connection) -> String {
        println!("Running handle_get_request()...");

        // 1. Create response
        let mut response = String::new();

        // 2. Check resource
        let resource = Self::get_resource(request);
        println!("Resource requested:\n{resource}");

        match resource.as_str() {
            "order-positions" => {
                response = OrderSystemResponse::RESPONSE_LINE_REPORT_OK.to_string();
                response.push_str("\n");
                response.push_str(&parse_query_to_json(conn, "SELECT * FROM order_position;"));
                response = response.replace("\"rows\":", "\"order-positions\":");
            },
            unknown_resource => {
                println!("Received an unknown resource: {}", unknown_resource);
                response = OrderSystemResponse::RESPONSE_LINE_RESOURCE_UNKNOWN.to_string();
                response.push_str("\n");
                response += &format!(r#"{{"resource": "{resource}"}}"#);
            }
        }

        println!("Exiting handle_get_request()...\n");
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

                // 3.3 Update product amounts
                Self::update_product_amounts(&id, conn, true);

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

        // 2. Get order id
        let order_id_result = Self::get_order_id_from_report(request);
        let order_id = match order_id_result {
            Ok(order_id) => order_id,
            Err(error) => {
                println!("Error {}", error);
                return OrderSystemResponse::RESPONSE_LINE_REPORT_FAIL.to_string();
            }
        };

        // 3. Check if order id exists
        match Order::check_if_order_exists(conn, &order_id) {
            Ok(_) => {
                println!("Order with id {order_id} exists!");
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
                return OrderSystemResponse::RESPONSE_LINE_REPORT_FAIL.to_string();
            }
        };

        // 4. Check status
        let status = Self::get_report_status(request);
        println!("Received status: {status}");

        match status.as_str() {
            OrderSystemApi::STATUS_OK => {
                println!("Order process was successful!");
                // 4.1.1 Update response
                response = OrderSystemResponse::RESPONSE_LINE_REPORT_OK.to_string();

                // 4.1.2 Update order status
                Order::update_order_status(conn, &order_id, "FINISHED");

                // 4.1.3 Update order positions
                Self::update_order_positions(request, conn);
            },

            OrderSystemApi::STATUS_FAIL => {
                println!("Order process was not successful!");
                // 4.2.1. Update response
                response = OrderSystemResponse::RESPONSE_LINE_REPORT_FAIL.to_string();

                // 4.2.2. Update order status 
                Order::update_order_status(conn, &order_id, "FAILED");

                // 4.2.3. Update product amounts 
                Self::update_product_amounts(&order_id, conn, false);
            },

            _ => {
                println!("Received unknown status... FAIL!");
                return OrderSystemResponse::RESPONSE_LINE_REPORT_FAIL.to_string();
            },

        }

        println!("Exiting handle_report_request()...");
        response
    }

    fn get_resource(request: &str) -> String {
        String::from(request.split(" ").collect::<Vec<&str>>()[1])
    }

    pub fn get_order_id_from_report(request: &str) -> Result<i64, ParseIntError> {
        let id_string = String::from(request.split(" ").collect::<Vec<&str>>()[1].split("/").collect::<Vec<&str>>()[1].trim());
        let id: i64 = match id_string.parse::<i64>() {
            Ok(id) => id,
            Err(error) => return Err(error),
        };

        Ok(id)
    }
    pub fn get_order_id_from_json(values: &serde_json::Value) -> i64 {
        serde_json::from_value(values["order_id"].to_owned()).unwrap()
    }

    pub fn get_report_status(request: &str) -> String {
        String::from(request.split("\n").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[1].trim())
    }

    pub fn get_report_body(request: &str) -> String {
        let mut request_vector: Vec<&str> = request.split("\n").collect::<Vec<&str>>();

        let length = request_vector.len();
        let mut i_start = length;
        let mut body = String::new();

        for i in 0..length {
            if request_vector[i].contains("{") {
                i_start = i;
                break;
            }
        }

        for i in i_start..length {
            body.push_str(request_vector[i]);
            body.push_str("\n");
        }

        //println!("Body:\n{}", body);
        body
    }

    pub fn update_order_positions(request: &str, conn: &Connection) {
        let body = &Self::get_report_body(request);
        let values: Value = serde_json::from_str(body).unwrap();
        let updated_positions = values["updated_positions"].as_array().unwrap();
    
        println!("Number of updated positions: {}", values["updated_positions"].as_array().unwrap().len());
    
        for order_position in updated_positions.into_iter() {
            let order_position_id: i64 = serde_json::from_value(order_position["position_id"].to_owned()).unwrap();
            let x: i64 = serde_json::from_value(order_position["position_x"].to_owned()).unwrap();
            let y: i64 = serde_json::from_value(order_position["position_y"].to_owned()).unwrap();
            
            let product_type_id: i64 = serde_json::from_value(order_position["product_type_id"].to_owned()).unwrap();
            
            println!("order id: {}", order_position["position_id"]);
            println!("x position: {}", order_position["position_x"]);
            println!("y position: {}", order_position["position_y"]);
            println!("product type id: {}\n", order_position["product_type_id"]);
            
            //OrderPosition::update_product_type_by_id(conn, &order_position_id, &product_type_id); // tralala
            OrderPosition::update_product_type_by_coordinates(conn, &x, &y, &product_type_id);
        }
    }

    pub fn update_product_amounts(order_id: &i64, conn: &Connection, decrease: bool) {
        println!("Updating order products from order: {order_id}...");
        // 1. Get all order_items
        let query = &format!("SELECT * FROM order_item WHERE order_id = {order_id};");
        let order_items = get_query_iterator(conn, &query);

        println!("Order items for order {order_id}:\n{:?}", order_items);
        print_rows_from_query(conn, &query);

        // 2. Calculate sign
        let sign: String = match decrease {
            true => "-".to_string(),
            false => "+".to_string(),
        };

        // 3. Update all product amounts
        for row in order_items {
            //let current_amount = &row[3];
            let current_amount = &row[3].parse::<i64>().unwrap();
            println!("current amount: {current_amount}");
            let current_product_id = &row[2];
            println!("Before updating amounts for product {current_product_id}:");
            print_rows_from_query(conn, &format!("SELECT * FROM product WHERE id = {current_product_id};"));

            /* 
            let query = &format!("
            UPDATE product 
            SET amount = (amount {sign} {current_amount})
            WHERE product_id = {current_product_id};");
            */

            let query = &format!("
            UPDATE product 
            SET amount = (amount {sign} {current_amount})
            WHERE id = {current_product_id};
            ");

            println!("query:\n{query}");
            let data = vec![current_amount];
            

            //conn.execute(&query, (data[0],),).unwrap();
            conn.execute(&query, ()).unwrap();

            println!("After updating amounts for product {current_product_id}:");
            print_rows_from_query(conn, &format!("SELECT * FROM product WHERE id = {current_product_id};"));
        }

    }
}
