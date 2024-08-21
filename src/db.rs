use rusqlite::{params, Connection};

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
            size INTEGER
            )", ()).expect("Table creation collections failed");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cards (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             collection_id INTEGER NOT NULL,
             card_number INTEGER NOT NULL,
             collected BOOLEAN NOT NULL,
             duplicates INTEGER DEFAULT 0,
             FOREIGN KEY (collection_id) REFERENCES collections(id)
        )", ()).expect("Table creation cards failed");
    }

    pub fn get_collections(&self) -> Vec<Collection> {
        let mut stmt = self.connection
            .prepare("SELECT id, name, size FROM collections")
            .expect("Statement Failed");

        let iter = stmt.query_map([], |row| {
            Ok(Collection::new(
                row.get(0)?,
                row.get(1)?)
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
                    row.get(4)?,
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

    fn generate_cards(&mut self, collection_id: u32, size: u32) -> Result<(), rusqlite::Error> {
        let tx = self.connection.transaction()?;

        {
            let mut stmt = tx.prepare("INSERT INTO cards (collection_id, card_number, collected) \
                                                     VALUES (?1, ?2, false)")?;

            for label in 1..=size {
                stmt.execute((collection_id, label))?;
            }
        }
        tx.commit()?;

        Ok(())
    }

    pub fn add_collection(&mut self, name: &String, size: u32) -> u32 {
        let last_id: u32 = {
            let mut stmt = self.connection
                .prepare("INSERT INTO collections (name, size) VALUES (?1,?2)")
                .expect("Statement Failed");
            stmt.execute((name, size)).expect("Query Failed");

            let mut stmt = self.connection
                .prepare("SELECT last_insert_rowid()")
                .expect("Statement Failed");

            stmt.query_row([], |row| {
                row.get(0)
            }).expect("Query Failed")
        };
        self.generate_cards(last_id, size).expect("Transaction Failed");
        last_id
    }

    pub fn delete_collection(&self, collection_id: u32) {
        self.connection.execute("DELETE FROM cards WHERE collection_id = ?1", [collection_id])
            .expect("Query Failed");
        self.connection.execute("DELETE FROM collections WHERE id = ?1", [collection_id])
            .expect("Query Failed");
    }

    pub fn get_collection_name(&self, collection_id: u32) -> String {
        let mut stmt = self.connection
            .prepare("SELECT name FROM collections WHERE id = ?1")
            .expect("Statement Failed");

        stmt.query_row([collection_id], |row| {
            row.get(0)
        }).expect("Query Failed")
    }

    pub fn get_max_label_for_collection(&self, collection_id: u32) -> u32 {
        let mut stmt = self.connection.prepare(
            "SELECT max(card_number) FROM cards WHERE collection_id = ?1"
        ).expect("Statement Failed");

        stmt.query_row([collection_id], |row| row.get(0)).unwrap_or(0)
    }

    pub fn add_card(&self, card_number: u32, collection_id: u32) -> u32 {
        self.connection.execute("INSERT INTO cards (collection_id, card_number, collected) VALUES (?1,?2,?3)",
                                params![collection_id,card_number,0]).expect("Query Failed");

        let mut stmt = self.connection
            .prepare("SELECT last_insert_rowid()")
            .expect("Statement Failed");

        stmt.query_row([], |row| {
            row.get(0)
        }).expect("Query Failed")
    }

    pub fn update_card_duplicates(&self, card: &mut Card) {
        self.connection.execute("UPDATE cards SET duplicates = ?1 WHERE id = ?2", [card.duplicates, card.id])
            .expect("Query Failed");
    }
}