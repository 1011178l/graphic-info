fn main() {
    let bytes = [0; 40];
    let graphic_info = GraphicInfo::new(&bytes);

    println!("{:?}", graphic_info);
}

#[derive(Debug, Default)]
struct GraphicInfo {
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
    fn new (bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 40 {
            return Err("There are 40 bytes for each graphic info chunk.");
        }

        let ret: Self = Default::default();

        Ok(ret)
    }
}
