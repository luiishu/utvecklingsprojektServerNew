#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rusqlite::{Connection, params, Result};

use crate::database::{order_position::OrderPosition, table::{print_rows_from_query, parse_query_to_json}};

pub fn hello_from_database_testing() {
    println!("Hello from database_testing!");
}

pub fn test(conn: &Connection) -> Result<()> {
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
    /* 
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
    */

    //print_rows_from_query(&conn, query).unwrap();
    //println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    
    //let query = "SELECT * FROM [order_position];";
    //print_rows_from_query(&conn, query).unwrap();
    //println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));

    /* 
    let query = "SELECT * FROM [order];";
    println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));
    let id = Order::get_oldest_ready_order_id(&conn).unwrap();
    println!("Id: {id}");

    
    print_rows_from_query(&conn, "SELECT * FROM order_item;").unwrap();

    println!("a:");
    print_rows_from_query(&conn, "SELECT * FROM order_item WHERE order_id = 2").unwrap();
    println!("a:");
    Order::get_all_order_items_from_order(&conn, &id).unwrap();

    //let query = "SELECT * FROM [order_item];";
    //println!("json string:\n{}", crate::database::table::parse_query_to_json(&conn, query));

    println!("HashMap: {:?}\n", Order::get_all_product_type_amounts_from_order(&conn, id));

    //let query = "SELECT id, order_id, product_id, amount FROM order_item WHERE order_id = 2;";
    let query = "SELECT o.id, o.order_id, o.product_id, p.product_type_id as product_type_id, pt.type, o.amount 
    FROM order_item o
    INNER JOIN product p ON p.id = o.product_id
    INNER JOIN product_type pt ON pt.id = p.product_type_id
    WHERE o.order_id = 2;";
    println!("==============================================================================================");
    print_rows_from_query(&conn, query).unwrap();
    println!("==============================================================================================");
    let query = "
    SELECT o.id as order_id, oi.id as order_item_id, pt.type as product_type, oi.amount as order_item_amount
    FROM order_item oi
    INNER JOIN [order] o ON o.id = oi.order_id
    INNER JOIN product p ON p.id = oi.product_id
    INNER JOIN product_type pt ON pt.id = p.product_type_id
    WHERE oi.order_id = 2;
    ";
    print_rows_from_query(&conn, query).unwrap();
    println!("==============================================================================================");
    println!("Total product amount for an order:");
    let query = "
    SELECT o.id as order_id, SUM(oi.amount) as total_product_amount
    FROM order_item oi
    INNER JOIN [order] o ON o.id = oi.order_id
    WHERE oi.order_id = 2;
    ";
    print_rows_from_query(&conn, query).unwrap();
    println!("==============================================================================================");
    println!("Product amount/product type for an order:");
    let query = "
    SELECT o.id as order_id, pt.type as product_type, SUM(oi.amount) as total_product_amount
    FROM order_item oi
    INNER JOIN [order] o ON o.id = oi.order_id
    INNER JOIN product p ON p.id = oi.product_id
    INNER JOIN product_type pt ON pt.id = p.product_type_id
    WHERE oi.order_id = 2
    GROUP BY pt.id;
    ";
    print_rows_from_query(&conn, query).unwrap();
    println!("==============================================================================================");
    let product_type_amounts = get_query_iterator(&conn, query);
    println!("{:?}", product_type_amounts);
    println!("==============================================================================================");
    println!("{}", Order::create_order_response(&conn, id));
    println!("==============================================================================================");
    println!("{}", Order::create_order_response_full(&conn, id));
    println!("==============================================================================================");
    println!("Printing all orders:");
    print_rows_from_query(&conn, "select * from [order];").unwrap();
    println!("==============================================================================================");
    println!("Printing oldest order that is ready:");
    print_rows_from_query(&conn, "SELECT * FROM [order] WHERE status = \"READY\" ORDER BY id ASC LIMIT 1;").unwrap();
    */

    //order_system::order_system::OrderSystem::hello_from_order_system();
    //order_system::order_system::OrderSystem::hello_from_order_system();

    /* 
    let query = "
    SELECT * FROM order_item
    INNER JOIN [order] ON order_item.order_id = [order].id
    WHERE [order].id = 2;";
    */

    //let query = "
    //SELECT * FROM [order_position];";

    //let query = "SELECT * FROM [order] WHERE id = 2;";
    //println!("{}", parse_query_to_json(conn, query));

    //let query = "SELECT * FROM [order_item] WHERE order_id = 2;";
    //println!("{}", parse_query_to_json(conn, query));
    //print_rows_from_query(conn, query).unwrap();




    //print_meta_data(conn)?;

    Ok(())
}

pub fn print_meta_data(conn: &Connection) -> Result<()> {
    print_rows_from_query(&conn, "SELECT name FROM sqlite_master where type='table';").unwrap();
    println!("Address--------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(address);").unwrap();
    println!("User-----------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(user);").unwrap();
    println!("Product type---------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product_type);").unwrap();
    println!("Product--------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product);").unwrap();
    println!("Product image---------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product_image);").unwrap();
    println!("Product review---------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(product_review);").unwrap();
    println!("Order----------------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info([order]);").unwrap();
    println!("Order item-----------------------------------------------------------------------------------------------------------------------------");
    print_rows_from_query(&conn, "pragma table_info(order_item);").unwrap();

    Ok(())
}