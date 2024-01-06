#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rusqlite::{Connection, params, Result};

//use server::database::table::{print_rows_from_query, parse_query_to_json}; //<--- funkar i fetch_han
//use crate::{request_line::RequestLine, database::table::get_query_iterator};

//use fetch_handler::fetch_handler;
use crate::{database::table::get_query_iterator, order_system};
//use server::fetch_handler::fetch_handler::*;

//use order_system::order_system;

use super::{
    table::{Table, print_rows_from_query}, 
    address::Address, user::User,
    product_type::ProductType, product::Product, 
    order::Order, order_item::OrderItem, order_position::OrderPosition, product_image::ProductImage, product_review::ProductReview, test::database_testing::hello_from_database_testing, product_brand::ProductBrand, product_category::ProductCategory
    };

pub fn hello_from_database() {
    crate::order_system::order_system::hello_from_order_system();
    let var = 1;

    println!("Hello from database folder!");
}
/* 
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
    OrderPosition::insert_initial_positions(&conn).unwrap();

    // Print data (testing)    
    hello_from_database_testing();
    crate::database::test::database_testing::test(&conn).unwrap();

    // Return database connection for server to use
    conn
}*/

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
        
        ProductBrand::create_table(conn)?;
        ProductCategory::create_table(conn)?;
        ProductType::create_table(conn)?;
        Product::create_table(conn)?;
        ProductImage::create_table(conn)?;
        ProductReview::create_table(conn)?;

        Order::create_table(conn)?;
        OrderItem::create_table(conn)?;
        OrderPosition::create_table(conn)?;        

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


        INSERT INTO product_brand (name)
        VALUES
        (\"Grupp4\"),
        (\"Grupp69\"),
        (\"Grupp420\"),
        (\"Grupp1337\");

        INSERT INTO product_category (name)
        VALUES
        (\"Hoodie\"),
        (\"T-shirt\"),
        (\"Pants\"),
        (\"Jeans\");

        --INSERT INTO product_image (file_name, file_name_hover, product_id)
        --VALUES
        --(\"/img/clothes/hoodie-yellow.png\", 420);
        --(\"/img/clothes/hoodie/hoodie-yellow-front.png\", \"/img/clothes/hoodie/hoodie-yellow-back.png\", 10),
        --(\"/img/clothes/hoodie/hoodie-blue-front.png\", \"/img/clothes/hoodie/hoodie-blue-back.png\", 11),
        --(\"/img/clothes/hoodie/hoodie-red-front.png\", \"/img/clothes/hoodie/hoodie-red-back.png\", 12),
        --(\"/img/clothes/hoodie/hoodie-green-front.png\", \"/img/clothes/hoodie/hoodie-green-back.png\", 13),
        --(\"/img/clothes/tshirt/tshirt-green-front.png\", \"/img/clothes/tshirt/tshirt-green-back.png\", 14),
        --(\"/img/clothes/tshirt/tshirt-blue-front.png\", \"/img/clothes/tshirt/tshirt-blue-back.png\", 15),
        --(\"/img/clothes/tshirt/tshirt-yellow-front.png\", \"/img/clothes/tshirt/tshirt-yellow-back.png\", 16),
        --(\"/img/clothes/tshirt/tshirt-red-front.png\", \"/img/clothes/tshirt/tshirt-red-back.png\", 17),
        --(\"/img/clothes/pants/pants-yellow-front.png\", \"/img/clothes/pants/pants-yellow-back.png\", 18),
        --(\"/img/clothes/pants/pants-red-front.png\", \"/img/clothes/pants/pants-red-back.png\", 19),
        --(\"/img/clothes/pants/pants-blue-front.png\", \"/img/clothes/pants/pants-blue-back.png\", 20),
        --(\"/img/clothes/pants/pants-green-front.png\", \"/img/clothes/pants/pants-green-back.png\", 21);
        --(\"/img/.png\", \"/img/.png\", 20);
           
        --INSERT INTO product (product_type_id, product_category_id, product_brand_id, name, product_rating, price, amount, product_image_id, description)
        --VALUES
           --(1, 1, 1, \"Red Dress\", 0, 6, 9, 1, \"A red dress.\"),
           --(1, 1, 1, \"Red apple\", 0, 6, 21, 1, \"A red apple.\"),
           --(2, 1, 1, \"Yellow Shoe\", 0, 6, 4, 1, \"A yellow shoe.\"),
           --(2, 1, 1, \"Yellow Stone\", 0, 6, 1, 1, \"A yellow stone. Must be important.\"),
           --(2, 1, 1, \"Bible\", 0, 6, 1, 1, \"A holy book.\"),
           --(3, 1, 1, \"Green apple\", 0, 6, 21, 1, \"A green apple.\"),
           --(4, 1, 1, \"Blue Man\", 0, 6, 4, 1, \"A blue man (what the-).\"),
           --(4, 1, 1, \"Blue Bikini\", 0, 6, 8, 1, \"A blue bikini.\"),
           --(3, 1, 1, \"Pink Pants\", 0, 6, 0, 1, \"A pair of pink pants.\");

        --INSERT INTO product (name, product_type_id, product_category_id, product_brand_id, product_rating, price, amount, product_image_id)        
        --VALUES
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 1),
            --(\"Cute Cat\", 4, 1, 1, 0, 199, 1, 2),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 3),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 4),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 5),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 6),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 7),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 8),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 9),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 10),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 11),
            --(\"Cute Cat\", 2, 1, 1, 0, 199, 1, 12);

           CREATE VIEW IF NOT EXISTS product_page
           AS SELECT 
           product.id AS productId,
           product.name AS productName,
           product_type.type AS color,
           product_category.name AS category,
           product_brand.name AS brand,
           product.product_rating AS review,
           product.price AS price,
           product.amount AS amount,
           product_image.file_name AS image

           FROM product
           INNER JOIN product_type ON product_type.id = product.product_type_id
           INNER JOIN product_category ON product_category.id = product.product_category_id
           INNER JOIN product_brand ON product_brand.id = product.product_brand_id
           INNER JOIN product_image ON product_image.id = product.product_image_id;

           CREATE VIEW IF NOT EXISTS detailed_product
           AS SELECT 
           product.id AS id,
           product.name AS productName,
           product_type.type AS color,
           product_category.name AS category,
           product_brand.name AS brand,
           product.product_rating AS review,
           product.price AS price,
           product.amount AS amount,
           product_image.file_name AS image,
           product_image.file_name_hover AS hoverImage

           FROM product
           INNER JOIN product_type ON product_type.id = product.product_type_id
           INNER JOIN product_category ON product_category.id = product.product_category_id
           INNER JOIN product_brand ON product_brand.id = product.product_brand_id
           INNER JOIN product_image ON product_image.id = product.product_image_id;   

        
        --INSERT into [order] (user_id, status)
        --VALUES (1, \"New\"), (1, \"READY\"), (2, \"READY\"), (2, \"New\"), (1, \"New\"), 
               --(2, \"New\"), (1, \"READY\"), (3, \"New\"), (1, \"READY\"), (1, \"New\");
        
        --insert into order_item (order_id, product_id, amount)
        --VALUES (1, 1, 1), (2, 1, 2), (2, 2, 3), (2, 3, 3), (2, 4, 3), (2, 6, 7), (2, 7, 1), (2, 8, 1),
               --(3, 3, 1), (4, 1, 1), (5, 7, 1), (6, 3, 1), (7, 8, 2), (8, 3, 1), (9, 7, 3), (10, 3, 1);        
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

