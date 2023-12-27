#![allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
};

use crate::{request_line::RequestLine, database::{database::{register_user, print_rows_from_user}, product::Product, table::{Table, get_query_iterator}, order::Order, order_item::OrderItem, user::User}, response::response::{ResponseLine, HttpResponseMessages, HttpResponseCode, HttpResponseCodes}};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use server::order_system::order_system::{self, OrderSystem};

use super::supported_resources::{SupportedResources, SupportedResourcesConstants};

pub struct PostHandler;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewOrder {
    pub order: Order,
    pub order_items: Vec<OrderItem>
}

impl PostHandler {
    pub fn handle_post_request(request: &str, conn: &Connection) -> String {
        let mut response = String::new();
        let request_line = RequestLine::new(&request);
    
        println!("Hello from handle_post_request_new!");
        //println!("Request received:\n{}", request);
        println!("Request line received:\n{}", request_line.to_string());

        let resource = Self::get_post_resource(&request_line.to_string());
        println!("Resource received: {resource}");

        if !Self::supported_resource(&resource) {
            println!("Resource '{resource}' is not supported.");
            return ResponseLine::get_response_line(HttpResponseCode::NOT_FOUND);
        }

        //let body = server::get_body_line2(&request.to_string());
        let body = OrderSystem::get_report_body(request);
        println!("Request body: {}", body);

        let response_code = match resource.as_str() {
            SupportedResources::USERS => Self::handle_users(&body, conn),
            SupportedResources::ORDERS => Self::handle_order(&body, conn),
            _ => {
                return ResponseLine::get_response_line(HttpResponseCode::NOT_FOUND);
            }
        };

        let status_line = ResponseLine::get_response_line(response_code);
        response = String::from(status_line);
        println!("Sending the following POST response:\n{response}");
        response
    }

    fn handle_order(body: &str, conn: &Connection) -> usize {
        // 1. Converting body to NewOrder struct
        let body = body.replace("order-items", "order_items");
        let new_order: NewOrder = serde_json::from_str(&body).unwrap();

        println!("Received the following order:\n{:?}", &new_order.order);
        println!("Order contains the following items:");
        for order_item in &new_order.order_items {
            println!("{:?}", order_item);
        }

        let newest_order_id = Order::get_newest_order_id(conn).unwrap();
        println!("Newest order id: {newest_order_id}");

        // 2. Inserting new order in database
        let query = "
        INSERT INTO [order] (user_id, product_amount, total_cost, order_date, order_timestamp, status)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);
        ";

        conn.execute(query, (
            new_order.order.user_id, 
            new_order.order.product_amount,
            new_order.order.total_cost,
            new_order.order.order_date,
            new_order.order.order_timestamp,
            new_order.order.status)
        ).unwrap();

        // 3. Inserting new order in database
        let newest_order_id = Order::get_newest_order_id(conn).unwrap();
        println!("Newest order id: {newest_order_id}");

        let query = "
        INSERT INTO [order_item] (order_id, product_id, amount, cost)
        VALUES (?1, ?2, ?3, ?4);
        ";

        for order_item in &new_order.order_items {
            conn.execute(query, (
                order_item.order_id, 
                order_item.product_id, 
                order_item.amount, 
                order_item.cost)
            ).unwrap();
        }

        HttpResponseCode::CREATED
    }

    fn handle_users(body: &str, conn: &Connection) -> usize {
        User::print_rows(conn).unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        let username = json["username"].to_string();
        let password = json["password"].to_string();
        let request_type = serde_json::from_value(json["request_type"].to_owned());
        if request_type.is_err() {
            return HttpResponseCode::BAD_REQUEST;
        }
        
        let request_type: String = request_type.unwrap();

        println!("Received username: {username}");
        println!("Received password: {password}");
        println!("Received request type: {request_type}");

        // 1. Deserialize body into a user
        let user = serde_json::from_str(&body);
        if user.is_err() {
            println!("Error parsing user");
            return HttpResponseCode::BAD_REQUEST;
        }

        let user: User = user.unwrap();

        // 2. Check if request is for login or registration
        match request_type.as_str() {
            "login" => return Self::login_user(&user, conn),
            "register" => return Self::register_user(&user, conn),
            _ => {
                println!("Received unknown request type: {request_type}");
                return HttpResponseCode::BAD_REQUEST;
            }
        }
        
        User::print_rows(conn).unwrap();
    }

    fn login_user(user: &User, conn: &Connection) -> usize {
        println!("Time to login user");

        // 1. Check if username exists
        let query = &format!("SELECT username FROM [user] WHERE username = '{}';", &user.username);
        let rows = get_query_iterator(conn, query);
        let length = rows.len();
        println!("Rows: {:?}", rows);

        if rows.is_empty() {
            println!("Username {} does not exist! Returning error...", &user.username);
            return HttpResponseCode::NOT_FOUND;
        }

        // 2. Check if username and password exists
        let query = &format!("
        SELECT username, password FROM [user] 
        WHERE username = '{}' AND password = '{}';", 
        &user.username, &user.password
        );
        let rows = get_query_iterator(conn, query);
        println!("Rows: {:?}", rows);

        if rows.is_empty() {
            println!("Username {} with password {} does not exist! Returning error...", &user.username, &user.password);
            return HttpResponseCode::BAD_REQUEST;
        }

        // 3. Return OK
        println!("Login was successful!");
        HttpResponseCode::OK  
    }
    
    fn register_user(user: &User, conn: &Connection) -> usize {
        println!("Time to register user");
        // 1. Try inserting user into database
        let query = "INSERT INTO [user] (username, password) VALUES (?1, ?2);";

        let insertion = conn.execute(query, (&user.username, &user.password));
        match insertion {
            Ok(number_of_rows) => {
                match number_of_rows {
                    0 => {
                        println!("No rows were changed. Insertion was NOT successful.");
                        return HttpResponseCode::BAD_REQUEST;
                    },
                    1 => {
                        println!("One row was changed. Insertion was successful!");
                    },
                    n => {
                        println!("{n} rows were changed. Insertion was NOT successful.");
                        return HttpResponseCode::BAD_REQUEST;
                    },
                }
            },

            Err(e) => {
                eprintln!("Found error inserting user {:?}:\n{e}", &user);
                return HttpResponseCode::CONFLICT;
            }
        }

        HttpResponseCode::CREATED
    }

    fn get_post_resource(request_line: &str) -> String {
        let uri = request_line.split(" ").collect::<Vec<&str>>();
        uri[1].split("/").last().unwrap().to_string()
    }

    fn supported_resource(resource: &str) -> bool {
        match resource {
            SupportedResources::USERS | SupportedResources::ORDERS => {},
            _ => {
                return false
            }
        }

        true
    }
}

