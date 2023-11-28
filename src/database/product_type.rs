#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::database::product_type;

use super::table::Table;

use rusqlite::{Connection, params, Result, Row};

pub struct ProductType {}

impl Table for ProductType {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_type (
                id INTEGER PRIMARY KEY,
                type         TEXT UNIQUE NOT NULL,
                description  TEXT
              );",
              (),
        )?;
        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        todo!()
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        println!("Running print_rows() from struct ProductType...");

        let mut stmt = conn.prepare("SELECT * FROM product_type")?;
        //println!("Statement created!");

        println!("Column names: {:?}", stmt.column_names());
        println!("Column count: {}", stmt.column_count());
    
        let iterator = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let product_type: String = row.get(1)?;

            let mut description: Option<String> = row.get(2)?;
            if description.is_none() {
                description = Some(String::from("null"));
            }
    
            let cols = vec![id.to_string(), product_type, description.unwrap()];
    
            Ok(cols)
        })?;
    
        //println!("Iterator created!");
    
        for row in iterator {
            println!("Found product type: [id, type, description]: {:?}", row.unwrap());
        }
    
        println!("Exiting print_rows() from struct ProductType...");
    
        Ok(())
    }
}
