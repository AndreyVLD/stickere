use std::path::Path;
use std::fs;
use rusqlite::{params, Connection};

use crate::card::Card;
use crate::collection::Collection;

/// A struct that handles database operations.
pub struct DbHandler {
    connection: Connection,
}

impl DbHandler {
    /// Creates a new `DbHandler` instance and initializes the database.
    ///
    /// # Arguments
    ///
    /// * `db_path` - A string slice that holds the path to the database file.
    ///
    /// # Returns
    ///
    /// * `DbHandler` - A new instance of `DbHandler`.
    pub fn new(db_path: &str) -> DbHandler {
        if let Some(parent_dir) = Path::new(db_path).parent() {
            // Check if the directory exists
            if !parent_dir.exists() {
                // Create the directory if it doesn't exist
                fs::create_dir_all(parent_dir).expect("Failed to create directory");
            }
        }
        
        let connection = Connection::open(db_path).expect("Database Connection failed");
        Self::init(&connection);
        Self { connection }
    }

    /// Initializes the database by creating necessary tables if they do not exist.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to the `Connection` object.
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

    /// Retrieves all collections from the database.
    ///
    /// # Returns
    ///
    /// * `Vec<Collection>` - A vector of `Collection` objects.
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

    /// Retrieves all cards from a specific collection.
    ///
    /// # Arguments
    ///
    /// * `id` - A `u32` representing the collection identifier.
    ///
    /// # Returns
    ///
    /// * `Vec<Card>` - A vector of `Card` objects.
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

    /// Updates the collected status of a card.
    ///
    /// # Arguments
    ///
    /// * `id` - A `u32` representing the card identifier.
    /// * `status` - A `bool` representing the collected status.
    pub fn update_card(&self, id: u32, status: bool) {
        let mut stmt = self.connection
            .prepare("UPDATE cards SET collected = ?1 WHERE id = ?2 ")
            .expect("Statement Failed");

        stmt.execute((status, id)).expect("Query Failed");
    }

    /// Generates cards for a specific collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - A `u32` representing the collection identifier.
    /// * `size` - A `u32` representing the number of cards to generate.
    ///
    /// # Returns
    ///
    /// * `Result<(), rusqlite::Error>` - A result indicating success or failure.
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

    /// Adds a new collection to the database.
    ///
    /// # Arguments
    ///
    /// * `name` - A reference to a `String` representing the collection name.
    /// * `size` - A `u32` representing the number of cards in the collection.
    ///
    /// # Returns
    ///
    /// * `u32` - The identifier of the newly added collection.
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

    /// Deletes a collection and its associated cards from the database.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - A `u32` representing the collection identifier.
    pub fn delete_collection(&self, collection_id: u32) {
        self.connection.execute("DELETE FROM cards WHERE collection_id = ?1", [collection_id])
            .expect("Query Failed");
        self.connection.execute("DELETE FROM collections WHERE id = ?1", [collection_id])
            .expect("Query Failed");
    }

    /// Retrieves the name of a specific collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - A `u32` representing the collection identifier.
    ///
    /// # Returns
    ///
    /// * `String` - The name of the collection.
    pub fn get_collection_name(&self, collection_id: u32) -> String {
        let mut stmt = self.connection
            .prepare("SELECT name FROM collections WHERE id = ?1")
            .expect("Statement Failed");

        stmt.query_row([collection_id], |row| {
            row.get(0)
        }).expect("Query Failed")
    }

    /// Retrieves the maximum card number for a specific collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - A `u32` representing the collection identifier.
    ///
    /// # Returns
    ///
    /// * `u32` - The maximum card number in the collection.
    pub fn get_max_label_for_collection(&self, collection_id: u32) -> u32 {
        let mut stmt = self.connection.prepare(
            "SELECT max(card_number) FROM cards WHERE collection_id = ?1"
        ).expect("Statement Failed");

        stmt.query_row([collection_id], |row| row.get(0)).unwrap_or(0)
    }

    /// Adds a new card to a specific collection.
    ///
    /// # Arguments
    ///
    /// * `card_number` - A `u32` representing the card number.
    /// * `collection_id` - A `u32` representing the collection identifier.
    ///
    /// # Returns
    ///
    /// * `u32` - The identifier of the newly added card.
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

    /// Updates the number of duplicates for a specific card.
    ///
    /// # Arguments
    ///
    /// * `card` - A mutable reference to the `Card` object that needs updated.
    pub fn update_card_duplicates(&self, card: &mut Card) {
        self.connection.execute("UPDATE cards SET duplicates = ?1 WHERE id = ?2", [card.duplicates, card.id])
            .expect("Query Failed");
    }
}