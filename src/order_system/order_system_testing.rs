use rusqlite::Connection;

use crate::order_system::order_system::OrderSystem;
#[allow(unused)]
use crate::order_system::order_system::{OrderSystemApi, OrderSystemApiConstants, OrderSystemRequestApi, OrderSystemRequest};

pub fn generate_order_request(conn: &Connection, n: &i32) {
    println!("Generating request from Order System...");

    for _i in 0..*n {
        //generate_process_request(conn);
        //generate_ok_report(conn);
        generate_fail_report(conn, &3);
    }
    
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

    println!("Printing all orders BEFORE generating order requests:");
    crate::database::table::print_rows_from_query(&conn, "select * from [order];").unwrap();

    generate_order_request(conn, n);

    println!("Printing all orders AFTER generating order requests:");
    crate::database::table::print_rows_from_query(&conn, "select * from [order];").unwrap();
    println!("------------------------------------------------------------------------------------");
}