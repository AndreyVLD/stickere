use rusqlite::Connection;

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
}