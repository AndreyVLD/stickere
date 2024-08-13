use rusqlite::Connection;

use crate::card::Card;
use crate::collection::Collection;

pub struct DbHandler {
    connection: Connection,
}

impl DbHandler {
    pub fn new(db_path: &str) -> DbHandler {
        let connection = Connection::open(db_path).expect("Database Connection failed");
        Self::init(&connection);
        Self { connection }
    }

    fn init(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            size INTEGER,
            description TEXT
            )", ()).expect("Table creation collections failed");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cards (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             collection_id INTEGER NOT NULL,
             card_number INTEGER NOT NULL,
             collected BOOLEAN NOT NULL,
             FOREIGN KEY (collection_id) REFERENCES collections(id)
        )"
            , ()).expect("Table creation cards failed");
    }

    pub fn get_collections(&self) -> Vec<Collection> {
        let mut stmt = self.connection
            .prepare("SELECT id, name, size FROM collections")
            .expect("Statement Failed");

        let iter = stmt.query_map([], |row| {
            Ok(Collection::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?)
            )
        }).expect("Query Failed");

        iter.flatten().collect()
    }

    pub fn get_cards_from_collection(&self, id: u32) -> Vec<Card> {
        let mut stmt = self.connection
            .prepare("SELECT * FROM cards WHERE collection_id = ?1 ORDER BY card_number")
            .expect("Statement Failed");

        let iter = stmt.query_map([id], |row| {
            Ok(
                Card::new(
                    row.get(2)?,
                    row.get(0)?,
                    row.get(3)?,
                )
            )
        }).expect("Query Failed");

        iter.flatten().collect()
    }

    pub fn update_card(&self, id: u32, status: bool) {
        let mut stmt = self.connection
            .prepare("UPDATE cards SET collected = ?1 WHERE id = ?2 ")
            .expect("Statement Failed");

        stmt.execute((status, id)).expect("Query Failed");
    }
}