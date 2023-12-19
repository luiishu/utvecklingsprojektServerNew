#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::database::product_type;
use super::table::Table;
use rusqlite::{Connection, params, Result, Row};

pub struct ProductCategory {}

impl Table for ProductCategory {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_category (
                id         INTEGER PRIMARY KEY,
                name TEXT
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
