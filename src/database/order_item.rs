#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::table::{Table, print_rows_from_query};
use rusqlite::{Connection, params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderItem {
    //pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub amount: i64,
    pub cost: i64,
}

impl Table for OrderItem {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS order_item (
                id INTEGER PRIMARY KEY,
                order_id INTEGER NOT NULL,
                product_id INTEGER NOT NULL,
                amount INTEGER DEFAULT 1,
                cost INTEGER DEFAULT 0,
                
                FOREIGN key(order_id) REFERENCES [order](id),
                FOREIGN key(product_id) REFERENCES product(id)
              );",
              (),
        )?;

        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        todo!()
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        let query = &format!("SELECT * FROM [order_item];");
        print_rows_from_query(conn, query)?;
        Ok(())
    }
}
