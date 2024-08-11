use rusqlite::{params, Connection, Result};
fn main() {
    let conn = Connection::open("db/my_database.db")
        .expect("Database Connection failed");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
            )", ()).expect("Table creation failed");
}
