#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use rusqlite::{Connection, params, Result, Row, types::ValueRef};

pub struct MasterTable {}

pub fn get_query_iterator(conn: &Connection, query: &str) -> Vec<Vec<String>> {
    let mut stmt = conn.prepare(query).unwrap();
    let column_count = stmt.column_count();

    let iterator = stmt.query_map([], |row| {
        let mut cols: Vec<String> = Vec::new();
        
        for i in 0..column_count {
            let data_result: Result<String, rusqlite::Error> = row.get(i);
            let mut data_error = false;

            let data = match data_result {
                Ok(data) => {
                    cols.push(String::from(&data));
                    data.to_string()
                },
                Err(error) => {                        
                    data_error = true;
                    //println!("Error found in iterator: {:?}", error);                        
                    String::new()
                }
            };
            
            if data_error {
                let data_result: Result<i64, rusqlite::Error> = row.get(i);
                let data = match data_result {
                    Ok(data) => {
                        data_error = false;
                        cols.push(data.to_string());
                        data
                    },
                    Err(error) => {
                        data_error = true;
                        //println!("Error found in iterator: {:?}", error);
                        0
                    }
                };
            }

            if data_error {
                let data_result: Result<f64, rusqlite::Error> = row.get(i);
                let data = match data_result {
                    Ok(data) => {
                        data_error = false;
                        cols.push(data.to_string());
                        data
                    },
                    Err(error) => {
                        data_error = true;                        
                        //println!("Error found in iterator: {:?}", error);
                        0.0
                    }
                };
            }

            if data_error {
                let data_result: Result<bool, rusqlite::Error> = row.get(i);
                let data = match data_result {
                    Ok(data) => {
                        cols.push(data.to_string());
                        data
                    },
                    Err(error) => {                        
                        //println!("Error found in iterator: {:?}", error);
                        cols.push("null".to_string()); // assumes all other options are exhausted
                        false
                    }
                };
            }
        }
                
        Ok(cols)
    }).unwrap();

    let mut rows: Vec<Vec<String>> = Vec::new();
    
    for row in iterator {
        rows.push(row.unwrap());
    }
    
    rows
}

pub fn print_rows_from_query(conn: &Connection, query: &str) -> Result<()> {
    let stmt = conn.prepare(query)?;
    let column_names = stmt.column_names();    
    let column_count = stmt.column_count();
    let iterator = get_query_iterator(conn, &query);

    for row in iterator {
        println!("Column names: {:?}: {:?}", column_names, row);
    }

    Ok(())
}

pub fn parse_query_to_json(conn: &Connection, query: &str) -> String {
    let stmt = conn.prepare(query).unwrap();
    let column_names = stmt.column_names(); 
    let column_count = stmt.column_count();

    let rows = get_query_iterator(conn, &query);

    let row_count = rows.len();
    //println!("Number of rows: {row_count}");
    //println!("Number of columns: {column_count}");

    let single_data = row_count == 1 && column_count == 1;
    
    let mut json_string = String::from("{");

    if !single_data {json_string.push_str("\n\"rows\": [\n");} 

    for row in 0..row_count {
        if !single_data {json_string.push_str("{");}
        
        for col in 0..column_count {
            json_string.push_str("\"");
            json_string.push_str(column_names[col]);
            json_string.push_str("\": ");
            //if single_data {println!("Curr json string: {json_string}")};

            let mut parse_error = true;            

            for i in 0..3 {
                match i { 
                    0 => {
                        let parse_result = &rows[row][col].parse::<i64>();
                        if parse_result.is_ok() {
                            parse_error = false;
                            break;         
                        }                        
                    }, 
                    1 => {
                        let parse_result = &rows[row][col].parse::<f64>();
                        if parse_result.is_ok() {
                            parse_error = false;
                            break;         
                        }                        
                    }, 
                    2 => {
                        let parse_result = &rows[row][col].parse::<bool>();
                        if parse_result.is_ok() {
                            parse_error = false;
                            break;         
                        }                        
                    }, 
                    _=> {},
                }
            }

            if parse_error && (&rows[row][col] != "null") {                
                json_string.push_str("\"");
                json_string.push_str(&rows[row][col]);
                json_string.push_str("\"");
            } else {
                json_string.push_str(&rows[row][col]);
            }
            
            if !single_data {json_string.push_str(", ")};
        }
        
        if !single_data {
            json_string.remove(json_string.len() - 1);
            json_string.remove(json_string.len() - 1);
            json_string.push_str("},\n");
        }
    }
    
    if !single_data {
        json_string.remove(json_string.len() - 1);
        json_string.remove(json_string.len() - 1);
        json_string.push_str("\n]");
    }
    json_string.push_str("\n}");
    json_string
}

pub fn get_single_data(conn: &Connection) {

}

pub trait Table {
    // CREATE
    fn create_table(conn: &Connection) -> Result<()>;
    fn insert(conn: &Connection, data: Vec<Vec<&str>>) -> Result<()>;

    // READ
    fn print_rows(conn: &Connection) -> Result<()>;

    fn print_rows_all(conn: &Connection, table_name: &str) -> Result<()> {
        println!("Running print_rows_all() from trait Table!");

        let query = format!("SELECT * FROM {table_name};");
        let stmt = conn.prepare(&query)?;
        
        let column_count = stmt.column_count();
        let column_names = stmt.column_names();
        
        println!("Column count: {}", column_count);
        println!("Column names: {:?}", &column_names);        

        let iterator = get_query_iterator(conn, &query);

        for row in iterator {
            println!("Column names: {:?}: {:?}", column_names, row);
        }

        println!("Exiting print_rows_all() from trait Table!");
        Ok(())
    }
}

impl Table for MasterTable {
    fn create_table(conn: &Connection) -> Result<()> {
        todo!()
    }

    fn insert(conn: &Connection, data: Vec<Vec<&str>>) -> Result<()> {
        todo!()
    }

    fn print_rows(conn: &Connection) -> Result<()> {
        todo!()
    }
}