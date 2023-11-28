#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::table::Table;
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
