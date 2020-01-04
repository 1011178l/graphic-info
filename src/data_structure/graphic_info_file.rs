extern crate sqlite;

use std::{
    fs::File,
    path::Path,
    io::{Read, Error, ErrorKind},
};
use crate::{
    data_structure::GraphicInfo,
    storage::Sqlite,
};
use sqlite::Value;
use bincode;

#[derive(Debug)]
pub struct GraphicInfoFile(File);

impl GraphicInfoFile {
    pub fn new(path: &Path) -> Result<Self, Error> {
        Ok(Self(File::create(&path)?))
    }

    pub fn open(path: &Path) -> Result<Self, Error> {
        let file = File::open(&path)?;
        if file.metadata()?.len() % 40 != 0 {
            return Err(Error::new(ErrorKind::Other, "Invalid input file size."));
        }

        Ok(Self(file))
    }

    pub fn show_info(&mut self) {
        println!("Number of Graphic Info: {}", self.count());
    }

    pub fn dump_into(&mut self, database: &Sqlite) -> Result<(), sqlite::Error> {
        let mut statement = database.connection.prepare(
            "INSERT INTO graphic_info (
                graphic_id, address, length, offset_x, offset_y, width, height, tile_east, tile_south, access, unknown0, unknown1, unknown2, unknown3, unknown4, map, binary
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);"
        )?.cursor();
        
        self.for_each(|graphic_info| {
            statement.bind(&[
                Value::Integer(graphic_info.id as i64), 
                Value::Integer(graphic_info.address as i64), 
                Value::Integer(graphic_info.length as i64),
                Value::Integer(graphic_info.offset_x as i64), 
                Value::Integer(graphic_info.offset_y as i64), 
                Value::Integer(graphic_info.width as i64), 
                Value::Integer(graphic_info.height as i64), 
                Value::Integer(graphic_info.tile_east as i64), 
                Value::Integer(graphic_info.tile_south as i64), 
                Value::Integer(graphic_info.access as i64),
                Value::Integer(graphic_info.unknown[0] as i64), 
                Value::Integer(graphic_info.unknown[1] as i64), 
                Value::Integer(graphic_info.unknown[2] as i64), 
                Value::Integer(graphic_info.unknown[3] as i64), 
                Value::Integer(graphic_info.unknown[4] as i64), 
                Value::Integer(graphic_info.map as i64),
                Value::Binary(bincode::serialize(&graphic_info).unwrap())
            ]).unwrap();

            statement.next().unwrap();
        });

        Ok(())
    }

    pub fn build_from(&mut self, database: &Sqlite) -> Result<(), Box<dyn std::error::Error>> {
        let mut cursor = database.connection.prepare("SELECT * FROM graphic_info")?.cursor();

        while let Some(row) = cursor.next()? {
            let graphic_info = GraphicInfo::from(row);

            database.update(row[0].as_integer().unwrap(), &bincode::serialize(&graphic_info)?)?;
            bincode::serialize_into(&self.0, &graphic_info)?;
        }

        Ok(())
    }
}

impl Iterator for GraphicInfoFile {
    type Item = GraphicInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 40];

        match self.0.read_exact(&mut buf) {
            Ok(_) => {
                let graphic_info: GraphicInfo = bincode::deserialize(&buf).unwrap();
                return Some(graphic_info);
            },
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GraphicInfoFile, GraphicInfo};
    use super::Sqlite;
    use std::path::Path;

    #[test]
    fn test_new() {
        assert!(GraphicInfoFile::open(&Path::new("./resources/GraphicInfo.test.bin")).is_ok());
    }

    #[test]
    fn test_new_failed() {
        assert!(GraphicInfoFile::open(&Path::new("./resources/GraphicInfo-broken.test.bin")).is_err())
    }

    #[test]
    fn test_iter() {
        let file = GraphicInfoFile::open(&Path::new("resources/GraphicInfo.test.bin")).unwrap();
        let blocks: Vec<GraphicInfo> = file.collect();

        assert_eq!(3, blocks.len());
    }

    #[test]
    fn test_dump_into() {
        let database = Sqlite::new(":memory:").unwrap();
        database.migrate().unwrap();
        let mut file = GraphicInfoFile::open(&Path::new("resources/GraphicInfo.test.bin")).unwrap();

        assert!(file.dump_into(&database).is_ok());
    }
}