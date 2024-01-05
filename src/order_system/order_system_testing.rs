#![allow(unused)]
use rusqlite::Connection;
use crate::order_system::order_system::OrderSystem;
use crate::order_system::order_system::{OrderSystemApi, OrderSystemApiConstants, OrderSystemRequestApi, OrderSystemRequest};

pub fn generate_order_request(conn: &Connection, n: &i32) {
    println!("Generating request from Order System...");
    //generate_process_request(conn);
    //println!("Printing all products BETWEEN generating order requests:");
    //crate::database::table::print_rows_from_query(&conn, "select * from [product];").unwrap();

    //generate_get_request(conn);
    //generate_patch_request(conn);
    generate_process_request(conn);
    
    for _i in 0..*n {
        //generate_process_request(conn);
        //generate_ok_report(conn, &2);
        //generate_fail_report(conn, &2);
    }    
}

pub fn generate_get_request(conn: &Connection) {
    println!("Generating GET request...");
    let request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_GET_ORDER_POSITIONS_FULL.to_string();
    OrderSystem::handle_request(&request, conn);   
}

pub fn generate_unknown_get_request(conn: &Connection) {
    println!("Generating bad GET request...");
    let mut request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_GET_ORDER_POSITIONS_FULL.to_string();
    request = request.replace("order-positions", "hankey");
    OrderSystem::handle_request(&request, conn);   
}

pub fn generate_patch_request(conn: &Connection) {
    println!("Generating PATCH request...");
    let mut request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_LINE_PATCH_ORDER_POSITIONS.to_string();
    //request = request.replace("order-positions", "hankey");

    let body = r#"
    {
        "updated_positions": [
            {
                "position_id": 1,
                "position_x": 1,
                "position_y": 1,
                "empty": null,
                "product_type_id": 4
            }
        ]
    }
    "#;

    request.push_str("\n");
    request.push_str(body);

    OrderSystem::handle_request(&request, conn);   
}

pub fn generate_process_request(conn: &Connection) {
    println!("Generating process request...");
    let request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_PROCESS_FULL;
    OrderSystem::handle_request(request, conn);   
}

pub fn generate_ok_report(conn: &Connection, id: &i64) {
    println!("Generating OK report...");
    
    let mut request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_REPORT_OK_NO_BODY.to_string();
    request = request.replace("/id ", &format!("/{id} "));

    let body = r#"
    {
        "updated_positions": [
            {
                "position_id": 1,
                "position_x": 1,
                "position_y": 1,
                "empty": null,
                "product_type_id": 2
            },
            {
                "position_id": 5,
                "position_x": 1,
                "position_y": 5,
                "empty": null,
                "product_type_id": 3
            },
            {
                "position_id": 10,
                "position_x": 2,
                "position_y": 4,
                "empty": false,
                "product_type_id": 4
            }
        ]
    }
    "#;

    request.push_str("\n");
    request.push_str(body);
    
    OrderSystem::handle_request(&request, conn);
}

pub fn generate_fail_report(conn: &Connection, id: &i64) {
    println!("Generating FAIL report...");
    
    let mut request = <OrderSystemRequest as OrderSystemRequestApi>::REQUEST_REPORT_FAIL_NO_BODY.to_string();
    request = request.replace("/id ", &format!("/{id} "));
    
    OrderSystem::handle_request(&request, conn);
}

pub fn test_order_system(conn: &Connection, n: &i32) {
    println!("------------------------------------------------------------------------------------");
    /* 
    println!("Testing order system...");
    println!("Order System Request Line 1: {}\n", OrderSystemRequest::REQUEST_LINE_PROCESS);
    
    println!("Order System Request Process Full:\n{}\n", OrderSystemRequest::REQUEST_PROCESS_FULL);
    println!("Order System Request Report (OK) Full:\n{}\n", OrderSystemRequest::REQUEST_PROCESS_FULL);
    println!("Order System Request Report (Fail) Full:\n{}\n", OrderSystemRequest::REQUEST_PROCESS_FULL);

    let request_empty = OrderSystemRequest::build_request("", "", "");
    let request_process = OrderSystemRequest::build_request(
        OrderSystemApi::METHOD_PROCESS,
        "orders/latest",
        OrderSystemApi::VERSION_1_0
    );

    println!("Order system request builder for process request:\n{}", request_empty);
    println!("Order system request builder for process request:\n{}", request_process);
    */

    //println!("Printing all orders BEFORE generating order requests:");
    //crate::database::table::print_rows_from_query(&conn, "select * from [order];").unwrap();
    println!("\nPrinting all order positions BEFORE generating order requests:");
    crate::database::table::print_rows_from_query(&conn, "select * from [order_position];").unwrap();
    //println!("Printing all products BEFORE generating order requests:");
    //crate::database::table::print_rows_from_query(&conn, "select * from [product];").unwrap();

    generate_order_request(conn, n);

    //println!("Printing all orders AFTER generating order requests:");
    //crate::database::table::print_rows_from_query(&conn, "select * from [order];").unwrap();
    println!("\nPrinting all order positions AFTER generating order requests:");
    crate::database::table::print_rows_from_query(&conn, "select * from [order_position];").unwrap();

    //println!("Printing all products AFTER generating order requests:");
    //crate::database::table::print_rows_from_query(&conn, "select * from [product];").unwrap();
    println!("------------------------------------------------------------------------------------");
}