#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::database::product_type;

use super::table::Table;

use rusqlite::{Connection, params, Result, Row};

pub struct ProductImage {}

impl Table for ProductImage {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_image (
                id         INTEGER PRIMARY KEY,
                file_name  TEXT NOT NULL,
                product_id INTEGER NOT NULL,

                FOREIGN KEY(product_id) REFERENCES product(id)
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
