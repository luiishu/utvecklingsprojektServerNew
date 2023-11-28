#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::database::address;

use super::table::Table;
use rusqlite::{Connection, params, Result, Row};

pub struct User {}

impl Table for User {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE table [user] (
                id INTEGER PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                email    TEXT,
                first_name TEXT,
                last_name TEXT,
                address_id INTEGER,
                FOREIGN KEY(address_id) REFERENCES address(id)
              );",    
            (),
        )?;

        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        todo!()
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        println!("Running print_rows() from struct User...");

        let mut stmt = conn.prepare("SELECT * FROM User")?;
        println!("Statement created!");
    
        let user_iterator = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let username: String = row.get(1)?;
            let password: String = row.get(2)?;
            let email: String = row.get(3)?;
            let first_name: String = row.get(4)?;
            let last_name: String = row.get(5)?;
            let address_id: i64 = row.get(6)?;
    
            let cols = vec![id.to_string(), username, password, email, first_name, last_name, address_id.to_string()];
    
            Ok(cols)
        })?;
    
        for user in user_iterator {
            println!("Found user: [id, username, password, email, first_name, last_name, address_id]: {:?}", user.unwrap());
        }
    
        println!("Exiting print_rows() from struct User...");

        Ok(())
    }
}

impl User {
    pub fn new() -> User {
        User {}
    }

    pub fn register_user() {

        
    }

    pub fn print_username_and_address(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        println!("Running print_username_and_address() from struct User...");

        let mut stmt = conn.prepare(
            "select [user].id as user_id, username, [user].address_id as user_address_id,
            address.* 
            from [user]
            inner join address
            on address.id = user.address_id;")?;
        
        println!("Statement created!");
        println!("Column names: {:?}", stmt.column_names());
        println!("Column count: {}", stmt.column_count());
    
    
        let iterator = stmt.query_map([], |row| {
            let user_id: i64 = row.get(0)?;
            let username: String = row.get(1)?;
            let user_address_id: i64 = row.get(2)?;
            let address_id: i64 = row.get(3)?;
            let country: String = row.get(4)?;
            let city: String = row.get(5)?;
            let zip_code: i64 = row.get(6)?;
    
            let cols = vec![user_id.to_string(), username, user_address_id.to_string(), address_id.to_string(),
            country, city, zip_code.to_string()];
    
            Ok(cols)
        })?;
    
        for record in iterator {
            println!("Found user: [user_id, username, user_address_id, address_id, country, city, zip_code]: {:?}", record.unwrap());
        }
    
        println!("Exiting print_username_and_address() from struct User...");

        Ok(())
    }
}