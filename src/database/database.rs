#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rusqlite::{Connection, params, Result};

use super::{
    table::{Table, print_rows_from_query}, 
    address::Address, user::User,
    product_type::ProductType, product::Product, 
    order::Order, order_item::OrderItem};

pub fn hello_from_database() {
    let var = 1;

    println!("Hello from database folder!");
}

pub fn init_database(in_memory: bool) -> Connection {    
    // Connect database in memory or file
    let conn = if in_memory {
        println!("\nInitializing database in memory...");
        Connection::open_in_memory().unwrap()
    } else {
        println!("\nInitializing database...");
        Connection::open_in_memory().unwrap()// should be open in file
    };

    // Enables foreign keys for connection
    //let _ = conn.set_db_config(rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true).unwrap();
    let _ = conn.set_db_config(rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, false).unwrap();

    // Create tables in database
    create_tables(&conn).unwrap();

    // Insert test data
    insert_test_data(&conn).unwrap();

    // Print data
    //Address::print_rows(&conn).unwrap();
    //<Address as Table>::print_rows_all(&conn, "address").unwrap();
    //print_rows_from_query(&conn, "SELECT * FROM address;").unwrap();

    //User::print_rows(&conn).unwrap();
    //<User as Table>::print_rows_all(&conn, "[user]").unwrap();
    //print_rows_from_query(&conn, "SELECT * FROM [user];").unwrap();
    
    //ProductType::print_rows(&conn).unwrap();
    //<ProductType as Table>::print_rows_all(&conn, "product_type").unwrap();
    //print_rows_from_query(&conn, "SELECT * FROM product_type;").unwrap();


    //Product::print_rows(&conn).unwrap();
    //<Product as Table>::print_rows_all(&conn, "product").unwrap();
    //print_rows_from_query(&conn, "SELECT * FROM product;").unwrap();
    //User::print_username_and_address(&conn).unwrap();
    //<User as Table>::print_rows_all(&conn, "product").unwrap();

    /*
    print_rows_from_query(&conn, 
        "select [user].id as user_id, username, [user].address_id as user_address_id, 
        address.* from [user] inner join address on address.id = user.address_id;").unwrap();
    */
    
    //print_rows_from_query(&conn, "SELECT * FROM [order];").unwrap();
    //print_rows_from_query(&conn, "SELECT * FROM order_item;").unwrap();

    //print_rows_from_query(&conn, "SELECT * FROM sqlite_master where type='table';").unwrap();
    //print_rows_from_query(&conn, "SELECT tbl_name FROM sqlite_master where type='table';").unwrap();
    /* 
    print_rows_from_query(&conn, "SELECT name FROM sqlite_master where type='table';").unwrap();
    println!("Address--------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(address);").unwrap();
    println!("User-----------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(user);").unwrap();
    println!("Product type---------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product_type);").unwrap();
    println!("Product--------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product);").unwrap();
    println!("Order----------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info([order]);").unwrap();
    println!("Order item-----------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(order_item);").unwrap();
    */

    let query = "SELECT * FROM address;";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let query = "SELECT * FROM [user];";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let query = "SELECT * FROM [product_type];";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let query = "SELECT * FROM product;";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let query = "SELECT * FROM [order];";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let query = "SELECT * FROM [order_item];";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    

    //print_rows_from_query(&conn, query).unwrap();
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    



    // Return database connection for server to use
    conn
}

pub fn connect_to_database_in_memory() {
    unimplemented!()
}

pub fn connect_to_database() {
    unimplemented!()
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            fname  TEXT NOT NULL,
            lname  TEXT NOT NULL
            )",
            (),
        )?;

        Address::create_table(conn)?;
        User::create_table(conn)?;
        //create_table_user(conn)?;
        
        ProductType::create_table(conn)?;
        Product::create_table(conn)?;

        Order::create_table(conn)?;
        OrderItem::create_table(conn)?;

        Ok(())
}

pub fn insert_test_data(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "insert into address (country, city, zip_code) 
        values 
           (\"Sweden\", \"Halmstad\", 12345), 
           (\"Sweden\", \"Malmö\", 13370),
           (\"Sweden\", \"Göteborg\", 42069);
        
        insert into [user] (username, password, email, first_name, last_name, address_id) 
        values 
           (\"ligma420\", \"abc123\", \"ligma420@email.cum\", \"Ligma\", \"Johnson\", 2), 
           (\"gauss1337\", \"password\", \"gau$$1337@email.cum\", \"Carl\", \"Gauss\", 1), 
           (\"hankey\", \"hankey\", \"hankey@email.cum\", \"Hankey\", \"...Hankey?\", 3);
        
        insert into product_type (type) 
        values
           (\"Red block\"),
           (\"Yellow block\"),
           (\"Green block\"),
           (\"Blue block\");
           
           
        INSERT INTO product (product_type_id, name, price, amount, description)
        VALUES
           (1, \"Red Dress\", 6, 9, \"A red dress.\"),
           (1, \"Red apple\", 6, 21, \"A red apple.\"),
           (2, \"Yellow Shoe\", 6, 4, \"A yellow shoe.\"),
           (2, \"Yellow Stone\", 6, 1, \"A yellow stone. Must be important.\"),
           (2, \"Bible\", 6, 1, \"A holy book.\"),
           (3, \"Green apple\", 6, 21, \"A green apple.\"),
           (4, \"Blue Man\", 6, 4, \"A blue man (what the-).\"),
           (5, \"Ble Bikini\", 6, 8, \"A blue bikini.\");
        
        
        insert into [order] (user_id)
        VALUES (1), (1), (2), (2), (1), (2), (1), (3), (1), (1);
        
        insert into order_item (order_id, product_id)
        VALUES (1, 1), (2, 2), (3, 3), (4, 1), (5, 7);        
    ")?;

    Ok(())
}

// user ---------------------------------------------------------------------------------------------
pub fn create_table_user(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id        INTEGER PRIMARY KEY,
            username  TEXT UNIQUE NOT NULL,
            password  TEXT NOT NULL
            )",
            (),
    )?;

    Ok(())
}

pub fn create_table_user_new(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id        INTEGER PRIMARY KEY,
            username  TEXT UNIQUE NOT NULL,
            password  TEXT NOT NULL
            )",
            (),
        ).unwrap();
}

pub fn register_user(conn: &Connection, data: Vec<Vec<&str>>) -> Result<()> {
    // 1. Check if username already exists in database

    // 2. Encrypt password

    // 3. Insert user to database

    println!("Trying to insert {} into table {}", data[1][1], data[0][1]);
    let query = format!("INSERT INTO {} ({}, {}) VALUES (?1, ?2)", data[0][1], data[1][0], data[2][0]);
    conn.execute(
        &query,
        (data[1][1], data[2][1]),
    )?;

    Ok(())
}

pub fn insert_user(conn: &Connection, data: Vec<Vec<&str>>) -> Result<()> {
    println!("Trying to insert {} into table {}", data[1][1], data[0][1]);
    conn.execute(
        "INSERT INTO user (username, password) VALUES (?1, ?2)",
        (data[1][1], data[2][1]),
    )?;

    Ok(())
}

pub fn print_rows_from_user(conn: &Connection) -> Result<()> {
    println!("Running print_rows_from_person()...");

    let mut stmt = conn.prepare("SELECT * FROM user")?;
    //println!("Statement created!");

    let user_iter = stmt.query_map([], |row| {
        //println!("id");
        let id: i64 = row.get(0)?;
        //println!("fname!");
        let username: String = row.get(1)?;
        //println!("lname!");
        let password: String = row.get(2)?;

        let cols = vec![id.to_string(), username, password];

        Ok(cols)
    })?;

    //println!("Iterator created!");

    for user in user_iter {
        println!("Found user [id, username, password]: {:?}", user.unwrap());
    }

    println!("Exiting print_rows_from_person()...");

    Ok(())
}

// product ---------------------------------------------------------------------------------------------

