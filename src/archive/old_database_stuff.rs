fn print_rows_users(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
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