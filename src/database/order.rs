#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


use std::collections::HashMap;

use super::{table::{Table, print_rows_from_query, get_query_iterator, parse_query_to_json}, order_position};
use rusqlite::{Connection, params, Result, Row};

pub struct Order {}

impl Table for Order {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS [order] (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                product_amount INTEGER DEFAULT 0,
                total_cost INTEGER DEFAULT 0,
                order_date TEXT DEFAULT CURRENT_DATE,
                order_timestamp TEXT DEFAULT CURRENT_TIMESTAMP,
                status TEXT NOT NULL DEFAULT \"NEW\",
                
                FOREIGN key(user_id) REFERENCES [user](id)
              );",
              (),
        )?;

        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        todo!()
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        todo!()
    }
}

impl Order {
    pub fn get_oldest_ready_order_id(conn: &rusqlite::Connection) -> Result<i64, String> {
        //let id = 0;
        let query = r#"SELECT id FROM [order] WHERE status = "READY" ORDER BY id ASC LIMIT 1;"#;
        print_rows_from_query(conn, query).unwrap();
        //id = table::q

        let data = get_query_iterator(conn, query);        
        println!("{:?}", data);

        let id = match data[0][0].parse::<i64>() {
            Ok(id) => id,
            Err(e) => return Err(e.to_string()),
        };

        println!("Returning id: {id}");
        
        Ok(id)
    }

    pub fn get_all_order_items_from_order(conn: &rusqlite::Connection, id: &i64) -> Result<i64, String> {
        let query = &format!("SELECT * FROM order_item WHERE order_id = {id};");
        print_rows_from_query(conn, query).unwrap();

        //println!("{}", query);
        Ok(1)
    }

    pub fn get_all_product_type_amounts_from_order(conn: &Connection, id: i64) -> HashMap<&'static str, i64> {
        let mut amounts: HashMap<&str, i64> = HashMap::from([
            ("Red", 0),
            ("Yellow", 0),
            ("Green", 0),
            ("Blue", 0),
        ]);

        amounts.insert("Red", 1);
        amounts.insert("Yellow", 2);
        amounts.insert("Green", 3);
        amounts.insert("Blue", 4);

        amounts
    }

    pub fn create_order_response(conn: &Connection, id: i64) -> String {
        let query = &format!("
        SELECT o.id as order_id, pt.type as product_type, SUM(oi.amount) as total_product_amount
        FROM order_item oi
        INNER JOIN [order] o ON o.id = oi.order_id
        INNER JOIN product p ON p.id = oi.product_id
        INNER JOIN product_type pt ON pt.id = p.product_type_id
        WHERE oi.order_id = {id}
        GROUP BY pt.id;
        ");
        
        let product_type_amounts = get_query_iterator(&conn, query);

        parse_query_to_json(conn, query)
    }

    #[allow(unused)]
    pub fn create_order_response_full(conn: &Connection, id: i64) -> String {                
        let mut response = String::new();

        let query_order_id = r#"SELECT id FROM [order] WHERE status = "READY" ORDER BY id ASC LIMIT 1;"#;

        let query_order_amount = &format!("
        SELECT pt.type as product_type, SUM(oi.amount) as total_product_amount
        FROM order_item oi
        INNER JOIN [order] o ON o.id = oi.order_id
        INNER JOIN product p ON p.id = oi.product_id
        INNER JOIN product_type pt ON pt.id = p.product_type_id
        WHERE oi.order_id = {id}
        GROUP BY pt.id;
        ");

        let query_order_positions = "SELECT * FROM [order_position];";
        
        //let product_type_amounts = get_query_iterator(&conn, query_amounts);
        //let order_positions = get_query_iterator(&conn, query_order_positions);
        
        let mut order_id = parse_query_to_json(conn, query_order_id);
        //order_id.remove(order_id.len() - 1);
        let order_id = order_id.replace("\n}", ",");

        let mut order_amount = parse_query_to_json(conn, query_order_amount);
        order_amount.remove(0);
        let order_amount = order_amount.replace("\n}", ",");
        let order_amount = order_amount.replace("rows", "amount");

        let mut order_positions = parse_query_to_json(conn, query_order_positions);
        order_positions.remove(0);
        let order_positions = order_positions.replace("rows", "positions");
        

        response = format!("{order_id}\n{order_amount}\n{order_positions}");

        response
    }


}