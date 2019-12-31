extern crate sqlite;

use std::path::Path;
use super::GraphicInfoFile;
use sqlite::{Connection, Value};

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(path);
        if !path.exists() {
            std::fs::File::create(path.to_str().unwrap())?;
        }

        let connection = sqlite::Connection::open(path).unwrap();
        
        Ok(Self{connection})
    }

    pub fn migrate(&self) -> Result<(), sqlite::Error> {
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

    pub fn insert(&self, graphic_info_file: GraphicInfoFile) {
        let mut statement = self.connection.prepare(
            "INSERT INTO graphic_info (
                graphic_id, address, length, offset_x, offset_y, width, height, tile_east, tile_south, access, unknown0, unknown1, unknown2, unknown3, unknown4, map
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);")
            .unwrap()
            .cursor();
        
            for gi in graphic_info_file {
                statement.bind(&[
                    Value::Integer(gi.id as i64), 
                    Value::Integer(gi.address as i64), 
                    Value::Integer(gi.length as i64),
                    Value::Integer(gi.offset_x as i64), 
                    Value::Integer(gi.offset_y as i64), 
                    Value::Integer(gi.width as i64), 
                    Value::Integer(gi.height as i64), 
                    Value::Integer(gi.tile_east as i64), 
                    Value::Integer(gi.tile_south as i64), 
                    Value::Integer(gi.unknown[0] as i64), 
                    Value::Integer(gi.unknown[1] as i64), 
                    Value::Integer(gi.unknown[2] as i64), 
                    Value::Integer(gi.unknown[3] as i64), 
                    Value::Integer(gi.unknown[4] as i64), 
                    Value::Integer(gi.map as i64)
                ]).unwrap();

                let _ = statement.next().unwrap();
            }
    }
}
