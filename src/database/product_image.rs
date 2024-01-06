#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::database::product_type;

use super::table::Table;

use rusqlite::{params, Connection, Result, Row};

pub struct ProductImage {}

impl Table for ProductImage {
    fn create_table(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_image (
                id         INTEGER PRIMARY KEY,
                file_name  TEXT NOT NULL,
                file_name_hover  TEXT,
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
        println!("Running print_rows() from struct ProductImage...");
        let query = &format!("SELECT * FROM [product_image];");
        crate::database::table::print_rows_from_query(conn, query)?;
        Ok(())
    }
}

impl ProductImage {
    pub fn insert_product_image(
        conn: &rusqlite::Connection,
        file_name: &str,
        file_name_hover: &str,
        product_id: i64,
    ) -> rusqlite::Result<()> {
        let query = "INSERT INTO product_image (file_name, file_name_hover, product_id) VALUES (?1, ?2, ?3)";
        match conn.execute(query, (file_name, file_name_hover, product_id)) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn get_latest_id(conn: &rusqlite::Connection) -> Result<i64, String> {
        //let id = 0;
        let query = r#"SELECT id FROM [product_image] ORDER BY id DESC LIMIT 1;"#;
        //print_rows_from_query(conn, query).unwrap();
        //id = table::q

        let data = crate::database::table::get_query_iterator(conn, query);
        println!("{:?}", data);
        if data.len() == 0 {
            return Err(String::from(
                "Error: there is no order that is ready for processing.",
            ));
        }
        let id = match data[0][0].parse::<i64>() {
            Ok(id) => id,
            Err(e) => return Err(e.to_string()),
        };

        println!("Returning id: {id}");

        Ok(id)
    }
}
