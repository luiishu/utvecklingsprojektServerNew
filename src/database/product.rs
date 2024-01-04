#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rusqlite::{Connection, params, Result as RusqliteResult, Row};

use crate::database::product_type;

use super::table::{Table, print_rows_from_query, get_query_iterator};

pub struct Product {}

impl Table for Product {
    fn create_table(conn: &Connection) -> RusqliteResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product (
                id INTEGER PRIMARY KEY,
                name                TEXT UNIQUE NOT NULL,
                product_type_id     INTEGER NOT NULL,
                product_category_id INTEGER,
                product_brand_id    INTEGER,
                product_image_id    INTEGER,
                product_rating      INTEGER DEFAULT 0,
                price               INTEGER NOT NULL,
                amount              INTEGER NOT NULL,
                description         TEXT,
                
                FOREIGN KEY(product_type_id) REFERENCES product_type(id),
                FOREIGN KEY(product_category_id) REFERENCES product_category(id),
                FOREIGN KEY(product_brand_id) REFERENCES product_brand(id)
                FOREIGN KEY(product_image_id) REFERENCES product_image(id)
                );",
                (),
        )?;
        
        Ok(())
    }

    fn insert(conn: &Connection, data: Vec<Vec<&str>>) -> RusqliteResult<()> {
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

    fn print_rows(conn: &Connection) -> RusqliteResult<()> {
        println!("Running print_rows() from struct Product...");
        let query = &format!("SELECT * FROM [product];");
        print_rows_from_query(conn, query)?;
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

    pub fn get_all_amounts(conn: &Connection) -> i64 {
        let query = "
            SELECT product.id, product_type_id, product_type.type, amount FROM product
            INNER JOIN product_type ON product_type.id = product.product_type_id;";
        print_rows_from_query(conn, query).unwrap();
        0
    }

    pub fn out_of_stock(conn: &Connection, color: &str) -> bool {
        Self::get_total_amount_by_color(conn, color) == 0
    }

    pub fn out_of_stock_product_exists(conn: &Connection) -> bool {
        let query = "SELECT * FROM product WHERE amount = 0;";
        let rows = get_query_iterator(conn, query);
        let length = rows.len();
        match length {
            0 => {
                return false
            },
            n => {
                println!("There is/are currently {n} product(s) that is/are out of stock");
                return true
            },
        }
    }

    pub fn get_total_amount_by_color(conn: &Connection, color: &str) -> i64 {
        let id = match color.to_lowercase().as_str() {
            "red" => 1,
            "yellow" => 2,
            "green" => 3,
            "blue" => 4,
            unknown_color => {
                println!("Received unknown color {unknown_color}");
                return -1
            }
        };

        let query = &format!("
            SELECT SUM(amount) FROM product
            WHERE product_type_id = {id};
            "
        );

        //println!("Query\n{query}");
        //print_rows_from_query(conn, query).unwrap();

        let rows = get_query_iterator(conn, query);
        //println!("rows: {:?}", rows);
        let length = rows.len();
        match length {
            0 => {
                return -1
            },
            1 => {
                let amount = rows[0][0].parse::<i64>();
                match amount {
                    Ok(amount) => return amount,
                    Err(e) => {
                        eprintln!("{e}");
                        return -1
                    }
                }
            },
            n => {
                return -1
            },
        }
    }

    pub fn get_amount_by_id(conn: &Connection, id: i64) -> i64 {        
        let query = &format!("
            SELECT amount FROM product
            WHERE id = {id};
            "
        );

        let rows = get_query_iterator(conn, query);
        let length = rows.len();
        match length {
            0 => {
                return -1
            },
            1 => {
                let amount = rows[0][0].parse::<i64>();
                match amount {
                    Ok(amount) => return amount,
                    Err(e) => {
                        eprintln!("{e}");
                        return -1
                    }
                }
            },
            n => {
                return -1
            },
        }
    }

    pub fn get_amount_by_name(conn: &Connection, name: &str) -> i64 {        
        let query = &format!("
            SELECT amount FROM product
            WHERE name = '{name}';
            "
        );

        let rows = get_query_iterator(conn, query);
        let length = rows.len();
        match length {
            0 => {
                return -1
            },
            1 => {
                let amount = rows[0][0].parse::<i64>();
                match amount {
                    Ok(amount) => return amount,
                    Err(e) => {
                        eprintln!("{e}");
                        return -1
                    }
                }
            },
            n => {
                return -1
            },
        }
    }

    pub fn get_product_count_by_color(conn: &Connection, color: &str) -> i64 {
        let id = match color.to_lowercase().as_str() {
            "red" => 1,
            "yellow" => 2,
            "green" => 3,
            "blue" => 4,
            unknown_color => {
                println!("Received unknown color {unknown_color}");
                return -1
            }
        };

        let query = &format!("
            SELECT COUNT(amount) FROM product
            WHERE product_type_id = {id};
            "
        );

        let rows = get_query_iterator(conn, query);
        let length = rows.len();
        match length {
            0 => {
                return -1
            },
            1 => {
                let amount = rows[0][0].parse::<i64>();
                match amount {
                    Ok(amount) => return amount,
                    Err(e) => {
                        eprintln!("{e}");
                        return -1
                    }
                }
            },
            n => {
                return -1
            },
        }
    }
    
    // UPDATE --------------------------------
    pub fn update_product() {
        unimplemented!()
    }

    pub fn update_product_amount_by_id(conn: &Connection, id: i64, amount: i64) -> RusqliteResult<()> {
        let mut sign = "";
        if amount < 0 {sign = "-";}
        else {sign = "+"};

        let query = &format!("UPDATE product SET amount = amount + {amount} WHERE id = {id}");
        match conn.execute(&query, ()) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        Ok(())
    }

    // DELETE --------------------------------
}