extern crate sqlite;

use std::{
    fs::File,
    path::Path,
    error::Error,
};
use sqlite::Connection;

pub struct Sqlite {
    pub connection: Connection,
}

impl Sqlite {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        match path {
            ":memory:" => {
                return Sqlite::open(path);
            }
            _ => {
                let path = Path::new(path);
                if !path.exists() {
                    File::create(path)?;
                }
        
                return Sqlite::open(path.to_str().unwrap());
            }
        }
    }

    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self{connection: Connection::open(path)?})
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
                map INTEGER,
                binary BINARY
            );")?;

            Ok(())
    }

    pub fn update(&self, id: i64, graphic_info: &[u8]) -> Result<(), sqlite::Error> {
        let mut statement = self.connection.prepare("UPDATE graphic_info SET binary = ? WHERE id = ?")?;
        statement.bind(1, graphic_info)?;
        statement.bind(2, id)?;
        statement.next()?;

        Ok(())
    }
}
