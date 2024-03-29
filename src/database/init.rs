#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rusqlite::{Connection, params, Result};
//use crate::{database::table::get_query_iterator, order_system};

pub fn init_database(in_memory: bool) -> Connection {    
    // Connect database in memory or file
    let conn = if in_memory {
        println!("\nInitializing database in memory...");
        Connection::open_in_memory().unwrap()
    } else {
        println!("\nInitializing database in file...");
        Connection::open("database.db").unwrap()
    };

    // Enables foreign keys for connection
    //let _ = conn.set_db_config(rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true).unwrap();
    let _ = conn.set_db_config(rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, false).unwrap();

    // Create tables in database
    super::database::create_tables(&conn).unwrap();

    // Insert test data
    if in_memory {
        super::database::insert_test_data(&conn).unwrap();
        super::order_position::OrderPosition::insert_initial_positions(&conn).unwrap();    
        super::product::Product::insert_product_from_json(&conn, "simple_products.json");
        super::product::Product::insert_product_from_json(&conn, "products.json");
    }

    //super::database::insert_test_data(&conn).unwrap();
    //super::order_position::OrderPosition::insert_initial_positions(&conn).unwrap();    
    //super::product::Product::insert_product_from_json(&conn, "simple_products.json");
    //super::product::Product::insert_product_from_json(&conn, "products.json");
    

    // Print data (testing)    
    //super::test::database_testing::hello_from_database_testing();
    //crate::database::test::database_testing::test(&conn).unwrap();

    // Return database connection for server to use
    conn
}