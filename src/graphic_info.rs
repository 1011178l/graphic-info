use serde::{Serialize, Deserialize};
use std::convert::From;

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

impl From<&[sqlite::Value]> for GraphicInfo {
    fn from(row: &[sqlite::Value]) -> Self {
        GraphicInfo {
            id: row[1].as_integer().unwrap() as u32,
            address: row[2].as_integer().unwrap() as u32,
            length: row[3].as_integer().unwrap() as u32,
            offset_x: row[4].as_integer().unwrap() as i32,
            offset_y: row[5].as_integer().unwrap() as i32,
            width: row[6].as_integer().unwrap() as u32,
            height: row[7].as_integer().unwrap() as u32,
            tile_east: row[8].as_integer().unwrap() as u8,
            tile_south: row[9].as_integer().unwrap() as u8,
            access: row[10].as_integer().unwrap() as u8,
            unknown: [
                row[11].as_integer().unwrap() as u8,
                row[12].as_integer().unwrap() as u8,
                row[13].as_integer().unwrap() as u8,
                row[14].as_integer().unwrap() as u8,
                row[15].as_integer().unwrap() as u8,
            ],
            map: row[16].as_integer().unwrap() as u32,
        }
    }
}


#[cfg(test)]
mod test {

    use super::GraphicInfo;
    use bincode;

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
