extern crate sqlite;

use std::path::Path;
use std::fs::File;
use std::error::Error;
use sqlite::Connection;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(path);
        if !path.exists() {
            File::create(path)?;
        }

        let connection = sqlite::Connection::open(path)?;
        
        Ok(Self{connection})
    }

    pub fn migrate(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DROP TABLE IF EXISTS graphic_info;
            CREATE TABLE graphic_info (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                graphic_id INTEGER, 
                address INTEGER, 
                length INTEGER, 
                offset_x INTEGER, 
                offset_y INTEGER, 
                width INTEGER, 
                height INTEGER, 
                tile_east INTEGER, 
                tile_south INTEGER, 
                access INTEGER, 
                unknown0 INTEGER,
                unknown1 INTEGER,
                unknown2 INTEGER,
                unknown3 INTEGER,
                unknown4 INTEGER,
                map INTEGER
            );")?;

            Ok(())
    }
}
