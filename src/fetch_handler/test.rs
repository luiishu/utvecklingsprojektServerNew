#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::fetch_handler::fetch_resources::{Admin, TableName};

pub fn test() {
    println!("Name of admin table: {}", Admin::TABLE_NAME);
}