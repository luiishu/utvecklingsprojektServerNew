use crate::database;
use rusqlite::Connection;
use server::database::table::{print_rows_from_query, get_query_iterator};
use crate::database::order::Order;

pub fn table_exists<'a>(conn: &Connection, table_name: &'a str) -> (&'a str, bool) {
    let query = &format!("SELECT tbl_name FROM sqlite_master WHERE type = 'table' AND tbl_name = '{table_name}';");
    let rows = get_query_iterator(&conn, query);
    (table_name, rows.len() == 1 && rows[0][0] == table_name)
}


#[cfg(test)]
mod test {
    use std::{fs, path::Path};    
    use crate::{database::product::Product, request_handler, response::response::{HttpResponseCode, HttpResponseCodes, ResponseLine}};

    use super::*;

    // Checks if a file named 'database.db' exists in project's root directory.
    #[test]
    fn database_exists() {
        //let conn = database::init::init_database(true);
        let database_path = "database.db";
        let database_exists = Path::new(database_path).exists();

        assert_eq!(true, database_exists);        
    }

    // Tests if all expected tables exist after initializing database.
    #[test] 
    fn test_all_tables_exist() {
        let conn = database::init::init_database(true);

        test_table_exists(&conn, "address");
        test_table_exists(&conn, "order_item");
        test_table_exists(&conn, "order_position");
        test_table_exists(&conn, "order");
        test_table_exists(&conn, "product_brand");
        test_table_exists(&conn, "product_category");
        test_table_exists(&conn, "product_image");
        test_table_exists(&conn, "product_review");
        test_table_exists(&conn, "product_type");
        test_table_exists(&conn, "product");
        test_table_exists(&conn, "user");

        assert_eq!(("unknown", false), table_exists(&conn, "unknown"));
    }

    fn test_table_exists(conn: &Connection, table_name: &str) {
        let conn = database::init::init_database(true);
        assert_eq!((table_name, true), table_exists(&conn, table_name));
    }

    // Tests if the correct ID for the oldest READY order is returned
    #[test]
    fn test_get_oldest_ready_order_id() {
        let conn = database::init::init_database(true);

        let query = "
            INSERT into [order] (user_id, status) 
            VALUES (1, \"FINISHED\"), (1, \"READY\"), (2, \"READY\");";
            
        conn.execute_batch(query).unwrap();
        let id = Order::get_oldest_ready_order_id(&conn).unwrap();
        assert_ne!(1, id);
        assert_eq!(2, id);        
        assert_ne!(3, id);        

        Order::update_order_status_to_processing(&conn, &id);
        let id = Order::get_oldest_ready_order_id(&conn).unwrap();        
        assert_ne!(1, id);        
        assert_ne!(2, id);
        assert_eq!(3, id);        
    }    

    // Tests if product types representing a red, yellow, green and blue block exist and are mapped to specific IDs.
    #[test]
    fn test_product_types() {
        let conn = database::init::init_database(true);
        let query = "SELECT id, type FROM product_type;";
        let mut rows = get_query_iterator(&conn, query);
        rows.sort();
        println!("{:?}", rows);

        assert_eq!(4, rows.len());
        assert_eq!(("1", "Red block"), (rows[0][0].as_str(), rows[0][1].as_str()));
        assert_eq!(("2", "Yellow block"), (rows[1][0].as_str(), rows[1][1].as_str()));
        assert_eq!(("3", "Green block"), (rows[2][0].as_str(), rows[2][1].as_str()));
        assert_eq!(("4", "Blue block"), (rows[3][0].as_str(), rows[3][1].as_str()));
        assert_ne!(("5", "Purple block"), (rows[3][0].as_str(), rows[3][1].as_str()));
    }

    // Tests that product amounts are updated correctly
    #[test]
    fn test_update_product_amount_by_id() {
        let conn = database::init::init_database(true);
        let query = "SELECT * FROM product LIMIT 4;";
        let n = get_query_iterator(&conn, query).len();

        for i in 1..=n {
            //println!("{i}");
            let id = i.try_into().unwrap();
            let amount = Product::get_amount_by_id(&conn, id);
            assert_eq!(0, amount);
        }

        for i in 1..=n {
            //println!("{i}");            
            let id = i.try_into().unwrap();
            Product::update_product_amount_by_id(&conn, id, 1);
            let amount = Product::get_amount_by_id(&conn, id);
            assert_eq!(1, amount);

            Product::update_product_amount_by_id(&conn, id, 0);
            let amount = Product::get_amount_by_id(&conn, id);
            assert_eq!(1, amount);

            Product::update_product_amount_by_id(&conn, id, -1);
            let amount = Product::get_amount_by_id(&conn, id);
            assert_eq!(0, amount);
        }

    }

    // Tests user registration by trying to register the same user twice.
    #[test]
    fn test_user_registration() {
        let conn = database::init::init_database(true);

        let request_line = String::from("POST /web_server/api/v1/users HTTP/1.1");
        let username = "username";
        let password = "password";
        let body = format!(r#"{{"username":"{username}","password":"{password}", "request_type":"register"}}"#);
        let request = format!("{request_line}\n{body}");

        let response = request_handler::handle_request(&request, &conn);
        let response_line = response.split("\n").next().unwrap();

        println!("Request sent:\n{request}");
        println!("Response received:\n{response}");

        assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CREATED), response_line);

        let response = request_handler::handle_request(&request, &conn);
        let response_line = response.split("\n").next().unwrap();

        println!("Request sent:\n{request}");
        println!("Response received:\n{response}");

        assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CONFLICT), response_line);
    }

        // Tests user login by trying to login before and after registration.
        #[test]
        fn test_user_login() {
            let conn = database::init::init_database(true);
    
            let request_line = String::from("POST /web_server/api/v1/users HTTP/1.1");
            let username = "username";
            let password = "password";
            
            let login_body = format!(r#"{{"username":"{username}","password":"{password}", "request_type":"login"}}"#);
            let registration_body = format!(r#"{{"username":"{username}","password":"{password}", "request_type":"register"}}"#);
            
            let login_request = format!("{request_line}\n{login_body}");
            let registration_request = format!("{request_line}\n{registration_body}");
    
            let response = request_handler::handle_request(&login_request, &conn);
            let response_line = response.split("\n").next().unwrap();
    
            println!("Request sent:\n{login_request}");
            println!("Response received:\n{response}");
    
            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::NOT_FOUND), response_line);

            let response = request_handler::handle_request(&registration_request, &conn);
            let response_line = response.split("\n").next().unwrap();
    
            println!("Request sent:\n{registration_request}");
            println!("Response received:\n{response}");
    
            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CREATED), response_line);

            let response = request_handler::handle_request(&login_request, &conn);
            let response_line = response.split("\n").next().unwrap();
    
            println!("Request sent:\n{login_request}");
            println!("Response received:\n{response}");
    
            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::OK), response_line);
        }

        // Tests user login by trying to login a user before and after registration.
        #[test]
        fn test_password_encryption() {
            let conn = database::init::init_database(true);
    
            let request_line = String::from("POST /web_server/api/v1/users HTTP/1.1");
            let username = "username";
            let password = "password";
            
            let login_body = format!(r#"{{"username":"{username}","password":"{password}", "request_type":"login"}}"#);
            let registration_body = format!(r#"{{"username":"{username}","password":"{password}", "request_type":"register"}}"#);

            let login_request = format!("{request_line}\n{login_body}");
            let registration_request = format!("{request_line}\n{registration_body}");

            let response = request_handler::handle_request(&registration_request, &conn);
            let response_line = response.split("\n").next().unwrap();
    
            println!("Request sent:\n{registration_request}");
            println!("Response received:\n{response}");
    
            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CREATED), response_line);

            let query = format!("SELECT password FROM [user] WHERE username = '{username}';");
            let encrypted_password = &get_query_iterator(&conn, &query)[0][0];

            println!("Unencrypted password:\n{password}");
            println!("Encrypted password:\n{encrypted_password}");

            assert_ne!(password, encrypted_password);

            let response = request_handler::handle_request(&login_request, &conn);
            let response_line = response.split("\n").next().unwrap();

            println!("Request sent:\n{login_request}");
            println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::OK), response_line);

            let encrypted_login_body = format!(r#"{{"username":"{username}","password":"{encrypted_password}", "request_type":"login"}}"#);
            let encrypted_login_request = format!("{request_line}\n{encrypted_login_body}");

            let response = request_handler::handle_request(&encrypted_login_request, &conn);
            let response_line = response.split("\n").next().unwrap();

            println!("Request sent:\n{encrypted_login_request}");
            println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::BAD_REQUEST), response_line);
        }

        // Tests order posting by posting an order multiple times after updating product amounts.
        #[test]
        fn test_order_posting() {
            let conn = database::init::init_database(true);
            let request_line = String::from("POST /web_server/api/v1/orders HTTP/1.1");

            let body = fs::read_to_string("json/order_posting_real.json").expect("Unable to open file");    
            let request = format!("{request_line}\n{body}");        
            
            let response = request_handler::handle_request(&request, &conn);
            let response_line = response.split("\n").next().unwrap();

            //println!("Request sent:\n{request}");
            //println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CONFLICT), response_line);

            Product::update_product_amount_by_id(&conn, 1, 2);

            let response = request_handler::handle_request(&request, &conn);
            let response_line = response.split("\n").next().unwrap();

            //println!("Request sent:\n{request}");
            //println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CONFLICT), response_line);

            Product::update_product_amount_by_id(&conn, 2, 2);

            let response = request_handler::handle_request(&request, &conn);
            let response_line = response.split("\n").next().unwrap();

            //println!("Request sent:\n{request}");
            //println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CONFLICT), response_line);

            Product::update_product_amount_by_id(&conn, 3, 4);

            let response = request_handler::handle_request(&request, &conn);
            let response_line = response.split("\n").next().unwrap();

            //println!("Request sent:\n{request}");
            //println!("Response received:\n{response}");

            assert_eq!(ResponseLine::get_response_line(HttpResponseCode::CREATED), response_line);
        }
}