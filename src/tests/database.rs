use crate::database;
use rusqlite::Connection;
use server::database::table::{print_rows_from_query, get_query_iterator};
use crate::database::order::Order;

pub fn table_exists<'a>(conn: &Connection, table_name: &'a str) -> (&'a str, bool) {
    let query = &format!("SELECT tbl_name FROM sqlite_master WHERE type = 'table' AND tbl_name = '{table_name}';");
    let rows = get_query_iterator(&conn, query);
    (table_name, rows.len() == 1 && rows[0][0] == table_name)
}


#[cfg(test)]
mod test {
    use std::path::Path;    
    use super::*;

    // Checks if a file named 'database.db' exists in project's root directory.
    #[test]
    fn database_exists() {
        //let conn = database::init::init_database(true);
        let database_path = "database.db";
        let database_exists = Path::new(database_path).exists();

        assert_eq!(true, database_exists);        
    }

    // Tests if all expected tables exist after initializing database.
    #[test] 
    fn test_all_tables_exist() {
        let conn = database::init::init_database(true);

        test_table_exists(&conn, "address");
        test_table_exists(&conn, "order_item");
        test_table_exists(&conn, "order_position");
        test_table_exists(&conn, "order");
        test_table_exists(&conn, "product_brand");
        test_table_exists(&conn, "product_category");
        test_table_exists(&conn, "product_image");
        test_table_exists(&conn, "product_review");
        test_table_exists(&conn, "product_type");
        test_table_exists(&conn, "product");
        test_table_exists(&conn, "user");

        assert_eq!(("unknown", false), table_exists(&conn, "unknown"));
    }

    fn test_table_exists(conn: &Connection, table_name: &str) {
        let conn = database::init::init_database(true);
        assert_eq!((table_name, true), table_exists(&conn, table_name));
    }

    // Tests if the correct ID for the oldest READY order is returned
    #[test]
    fn test_get_oldest_ready_order_id() {
        let conn = database::init::init_database(true);

        let query = "
            INSERT into [order] (user_id, status) 
            VALUES (1, \"FINISHED\"), (1, \"READY\"), (2, \"READY\");";
            
        conn.execute_batch(query).unwrap();
        let id = Order::get_oldest_ready_order_id(&conn).unwrap();
        assert_ne!(1, id);
        assert_eq!(2, id);        
        assert_ne!(3, id);        

        Order::update_order_status_to_processing(&conn, &id);
        let id = Order::get_oldest_ready_order_id(&conn).unwrap();        
        assert_ne!(1, id);        
        assert_ne!(2, id);
        assert_eq!(3, id);        
    }    

    // Tests if product types representing a red, yellow, green and blue block exist and are mapped to specific IDs.
    #[test]
    fn test_product_types() {
        let conn = database::init::init_database(true);
        let query = "SELECT id, type FROM product_type;";
        let mut rows = get_query_iterator(&conn, query);
        rows.sort();
        println!("{:?}", rows);

        assert_eq!(4, rows.len());
        assert_eq!(("1", "Red block"), (rows[0][0].as_str(), rows[0][1].as_str()));
        assert_eq!(("2", "Yellow block"), (rows[1][0].as_str(), rows[1][1].as_str()));
        assert_eq!(("3", "Green block"), (rows[2][0].as_str(), rows[2][1].as_str()));
        assert_eq!(("4", "Blue block"), (rows[3][0].as_str(), rows[3][1].as_str()));
        assert_ne!(("5", "Purple block"), (rows[3][0].as_str(), rows[3][1].as_str()));
    }
}