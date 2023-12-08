#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::database::product_type;
use super::table::Table;
use rusqlite::{Connection, params, Result, Row};

pub struct ProductReview {}

impl Table for ProductReview {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_image (
                id         INTEGER PRIMARY KEY,
                rating INTEGER DEFAULT 0,
                product_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,

                FOREIGN KEY(product_id) REFERENCES product(id),
                FOREIGN KEY(user_id) REFERENCES [user](id)
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
