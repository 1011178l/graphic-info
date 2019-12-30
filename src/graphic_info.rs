use std::io::Cursor;
use scroll::IOread;

#[derive(Debug)]
pub struct GraphicInfo {
    id: u32,
    address: u32,
    length: u32,
    offset_x: i32,
    offset_y: i32,
    width: u32,
    height: u32,
    tile_east: u8,
    tile_south: u8,
    unknown: [u8; 5],
    map: u32,
}

impl GraphicInfo {
    pub fn new (bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 40 {
            return Err("There are 40 bytes for each graphic info chunk.");
        }

        let mut cursor = Cursor::new(bytes);

        let id = cursor.ioread::<u32>().unwrap();
        let address = cursor.ioread::<u32>().unwrap();
        let length = cursor.ioread::<u32>().unwrap();
        let offset_x = cursor.ioread::<i32>().unwrap();
        let offset_y = cursor.ioread::<i32>().unwrap();
        let width = cursor.ioread::<u32>().unwrap();
        let height = cursor.ioread::<u32>().unwrap();
        let tile_east = cursor.ioread::<u8>().unwrap();
        let tile_south = cursor.ioread::<u8>().unwrap();

        let mut unknown = [0; 5];
        for i in 0..5 {
            unknown[i] = cursor.ioread::<u8>().unwrap();
        }

        let map = cursor.ioread::<u32>().unwrap();
        
        Ok(Self {
            id, address, length, offset_x, offset_y, width, height, tile_east, tile_south, unknown, map,
        })
    }
}