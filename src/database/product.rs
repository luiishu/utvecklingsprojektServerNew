#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::os::windows::io::InvalidHandleError;

use rusqlite::{Connection, params, Result, Row};

use crate::database::product_type;

use super::table::Table;

pub struct Product {}

impl Table for Product {
    fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product (
                id INTEGER PRIMARY KEY,
                name         TEXT NOT NULL,
                product_type_id TEXT NOT NULL,
                price        INTEGER NOT NULL,
                amount       INTEGER NOT NULL,
                description  TEXT,
                
                FOREIGN key(product_type_id) REFERENCES product_type(id)              
                );",
                (),
        )?;
        
        Ok(())
    }

    fn insert(conn: &Connection, data: Vec<Vec<&str>>) -> Result<()> {
        println!("Trying to insert {} into table {}", data[1][1], data[0][1]);
        let query = format!("INSERT INTO {} ({}, {}, {}, {}) VALUES (?1, ?2, ?3, ?4)",
                                  data[0][1], data[1][0], data[2][0], data[3][0], data[4][0]);
        
        println!("Query: {}", query);
        conn.execute(
            &query,
            (data[1][1], data[2][1].parse::<i64>().unwrap(), data[3][1].parse::<i64>().unwrap(), data[4][1]),
        )?;

        println!("Exiting insert from struct Product...");
        Ok(())        
    }

    fn print_rows(conn: &Connection) -> Result<()> {
        println!("Running print_rows() from struct Product...");

        let mut stmt = conn.prepare("SELECT * FROM product")?;
        println!("Statement created!");
    
        let product_iter = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let product_type: String = row.get(2)?;
            let price: i64 = row.get(3)?;
            let amount: i64 = row.get(4)?;
            let description: String = row.get(5)?;
    
            let cols = vec![id.to_string(), name, product_type, price.to_string(), amount.to_string(), description];
    
            Ok(cols)
        })?;
    
        println!("Iterator created!");
    
        for product in product_iter {
            println!("Found product: [id, name, type, price, amount, description]: {:?}", product.unwrap());
        }
    
        println!("Exiting print_rows() from struct Product...");
    
        Ok(())
    }
}

impl Product {
    pub fn new() -> Product {
        Product {}
    }

    // CREATE --------------------------------
    pub fn register_product() {
        todo!()
    }

    pub fn insert_product(&self) {
        todo!()
    }

    // READ --------------------------------
    pub fn get_latest_product(conn: &Connection) -> String {
        let mut stmt = conn.prepare("SELECT * FROM product ORDER BY id DESC LIMIT 1").unwrap();

        let mut product_iter = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;   
            let price: i64 = row.get(2)?;
            let amount: i64 = row.get(3)?;
            let description: String = row.get(4)?;
    
            let cols = vec![id.to_string(), name, price.to_string(), amount.to_string(), description];
    
            Ok(cols)
        }).unwrap();

        return format!("{:?}", product_iter.next().as_ref().unwrap().as_ref().unwrap());
    }

    pub fn get_product_iterator(conn: &Connection, query: &str) -> Vec<Vec<String>> {
        let mut stmt = conn.prepare(query).unwrap();
        let mut col_counter: u32 = 0;

        let product_iter = stmt.query_map([], |row| {
            let mut cols: Vec<String> = Vec::new();

            for i in 0..10 {
                let data_result: Result<String, rusqlite::Error> = row.get(i);
                let mut data_error = false;

                let data = match data_result {
                    Ok(data) => {
                        cols.push(String::from(&data));
                        col_counter += 1;
                        data.to_string()
                    },
                    Err(error) => {                        
                        println!("Error found in product iterator: {:?}", error);
                        if error.to_string().contains("Invalid column index") { // no use going further since there are no more columns
                            println!("OMG I FOUND AN INVALID COLUMN INDEX LET'S HAVE A BREAK!:\n {:?}", error);
                            break;
                        }
                        data_error = true;
                        String::new()
                    }
                };

                if data_error {
                    let data_result: Result<i64, rusqlite::Error> = row.get(i);
                    let data: i64 = match data_result {
                        Ok(data) => {
                            cols.push(data.to_string());
                            col_counter += 1;
                            data
                        },
                        Err(error) => {                        
                            println!("Error found in product iterator: {:?}", error);
                            0
                        }
                    };
                }                
            } 

            println!("Col counter: {}", col_counter);
    
            Ok(cols)
        }).unwrap();

        let mut vector: Vec<Vec<String>> = Vec::new();

        for data in product_iter {
            vector.push(data.unwrap());
        }

        vector
    }

    pub fn get_oldest_product(conn: &Connection) -> String {
        let query = "SELECT * FROM product ORDER BY id ASC LIMIT 1";
        let product_iter = Product::get_product_iterator(conn, query);
        return format!("{:?}", product_iter.get(0).unwrap());
    }

    pub fn get_cheapest_product(conn: &Connection) -> String {
        println!("Running cheapest!");
        let query = "SELECT * FROM product ORDER BY price ASC LIMIT 1";
        let product_iter = Product::get_product_iterator(conn, query);
        return format!("{:?}", product_iter.get(0).unwrap());
    }
    

    // UPDATE --------------------------------
    pub fn update_product() {
        unimplemented!()
    }

    // DELETE --------------------------------
}