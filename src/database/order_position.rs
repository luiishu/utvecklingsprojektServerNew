#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::format;

use super::{table::Table, order::Order};
use rusqlite::{Connection, params, Result, Row};

pub struct OrderPosition {}

impl Table for OrderPosition {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS order_position (
                ID INTEGER PRIMARY KEY,
                position_x INTEGER NOT NULL,
                position_y INTEGER NOT NULL,
                empty BOOL,
                product_type_id INTEGER,
                
                FOREIGN KEY(product_type_id) REFERENCES product_type(id)
              );
              ",
              (),
        )?;

        Ok(())
    }

    fn insert(conn: &rusqlite::Connection, data: Vec<Vec<&str>>) -> rusqlite::Result<()> {
        //println!("Trying to insert {} into table {}", data[1][1], data[0][1]);
        
        let query = format!("INSERT INTO order_position ({}, {}, {}, {}) VALUES (?1, ?2, ?3, ?4)",
         data[0][0], data[1][0], data[2][0], data[3][0]);
        
        conn.execute(
            &query,
            (data[0][1], data[1][1], data[2][1], data[3][1]),
        )?;

        Ok(())
    }

    fn print_rows(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        todo!()
    }
}

impl OrderPosition {
    pub fn insert_initial_positions(conn: &Connection) -> Result<()>  {
        let x_max = 4;
        let y_max = 6;
        

        for x in 1..(x_max + 1) {
            let data: Vec<Vec<&str>> = vec![vec!["", ""] ];
            let x_str = &x.to_string();

            for y in 1..(y_max + 1) {
                println!("Inserting new position with coordinates ({x}, {y})!");
                let y_str = &y.to_string();

                let data: Vec<Vec<&str>> = vec![
                    vec!["position_x", x_str],
                    vec!["position_y", y_str],
                    vec!["empty", "false"],
                    vec!["product_type_id", "1"]
                    ];
                
                OrderPosition::insert(conn, data)?;
            }
        }

        Ok(())
    }

    pub fn update_product_type_by_id(conn: &Connection, order_position_id: &i64, product_type_id: &i64) -> Result<()> {
        let query = &format!(
            "UPDATE order_position 
            SET product_type_id = {product_type_id} 
            WHERE id = {order_position_id};"
        );

        conn.execute(query, ())?;
        Ok(())
    }

    pub fn update_product_type_by_coordinates(conn: &Connection, x: &i64, y: &i64, product_type_id: &i64) -> Result<()> {
        let query = &format!(
            "UPDATE order_position 
            SET product_type_id = {product_type_id} 
            WHERE position_x = {x} AND position_y = {y};"
        );

        conn.execute(query, ())?;
        Ok(())
    }
}