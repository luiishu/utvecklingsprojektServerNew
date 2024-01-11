#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::table::Table;

use rusqlite::{Connection, params, Result, Row};

pub struct Address {
    row1: String,

}

pub enum AddressRows {
    Id(i64),
    Country(String),
    City(String),
    ZipCode(i64)    
}

impl Table for Address {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS address (
                id INTEGER PRIMARY KEY,
                country TEXT NOT NULL,
                city TEXT NOT NULL,
                zip_code INTEGER NOT NULL
            );",    
            (),
        )?;

        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        todo!()
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        println!("Running print_rows() from struct Address...");

        let mut stmt = conn.prepare("SELECT * FROM address")?;
        println!("Statement created!");
    
        let address_iterator = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let country: String = row.get(1)?;
            let city: String = row.get(2)?;
            let zip_code: i64 = row.get(3)?;
    
            let cols = vec![id.to_string(), country, city, zip_code.to_string()];
    
            Ok(cols)
        })?;
    
        for address in address_iterator {
            println!("Found address: [id, country, city, zip_code]: {:?}", address.unwrap());
        }
    
        println!("Exiting print_rows() from struct Address...");

        Ok(())
    }
}

impl Address {
    fn print_rows_enum(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        println!("Running print_rows() from struct Address...");

        let mut stmt = conn.prepare("SELECT * FROM address")?;
        println!("Statement created!");
    
        let address_iterator = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let country: String = row.get(1)?;
            let city: String = row.get(2)?;
            let zip_code: i64 = row.get(3)?;

            //let Address(Id) = row.get(0)?;
    
            let cols = vec![id.to_string(), country, city, zip_code.to_string()];
    
            Ok(cols)
        })?;
    
        //println!("Iterator created!");
    
        for address in address_iterator {
            println!("Found address: [id, country, city, zip_code]: {:?}", address.unwrap());
        }
    
        println!("Exiting print_rows() from struct Address...");

        Ok(())
    }
}