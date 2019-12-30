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
    access: u8,
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