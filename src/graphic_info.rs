use std::io::Cursor;
use scroll::IOread;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GraphicInfo {
    pub id: u32,
    pub address: u32,
    pub length: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: u32,
    pub height: u32,
    pub tile_east: u8,
    pub tile_south: u8,
    pub access: u8,
    pub unknown: [u8; 5],
    pub map: u32,
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
        let access = cursor.ioread::<u8>().unwrap();

        let mut unknown = [0; 5];
        for i in 0..5 {
            unknown[i] = cursor.ioread::<u8>().unwrap();
        }

        let map = cursor.ioread::<u32>().unwrap();
        
        Ok(Self {
            id, address, length, offset_x, offset_y, width, height, tile_east, tile_south, access, unknown, map,
        })
    }
}

#[cfg(test)]
mod test {

    use super::GraphicInfo;
    use bincode;

    #[test]
    fn test_new() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00,
            0xe0, 0xff, 0xff, 0xff, 0xe8, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x00,
            0x2f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xe7, 0x03, 0x00, 0x00
        ];
        let graphic_info = GraphicInfo::new(&bytes).unwrap();
    
        assert_eq!(graphic_info.id, 0);
        assert_eq!(graphic_info.address, 0);
        assert_eq!(graphic_info.length, 424);
        assert_eq!(graphic_info.offset_x, -32);
        assert_eq!(graphic_info.offset_y, -24);
        assert_eq!(graphic_info.width, 64);
        assert_eq!(graphic_info.height, 47);
        assert_eq!(graphic_info.tile_east, 1);
        assert_eq!(graphic_info.tile_south, 1);
        assert_eq!(graphic_info.access, 1);
        assert_eq!(graphic_info.unknown, [0, 0, 0, 0, 0]);
        assert_eq!(graphic_info.map, 999);
    }

    #[test]
    fn test_serialize() {
        let graphic_info = GraphicInfo {
            id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47, tile_east: 1, tile_south: 1, access: 1, unknown: [0, 0, 0, 0, 0], map: 999
        };

        let bytes = bincode::serialize(&graphic_info).unwrap();
        let expect =  [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00,
            0xe0, 0xff, 0xff, 0xff, 0xe8, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x00,
            0x2f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xe7, 0x03, 0x00, 0x00
        ];

        assert!(bytes.iter().eq(expect.iter()));
    }

    #[test]
    fn test_deserialize()
    {
        let bytes =  [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00,
            0xe0, 0xff, 0xff, 0xff, 0xe8, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x00,
            0x2f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xe7, 0x03, 0x00, 0x00
        ];
        let graphic_info: GraphicInfo = bincode::deserialize(&bytes).unwrap();

        assert_eq!(graphic_info, GraphicInfo {
            id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47, tile_east: 1, tile_south: 1, access: 1, unknown: [0, 0, 0, 0, 0], map: 999
        });
    }
}
