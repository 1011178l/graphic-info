fn main() {
    println!("Hello, world!");
}

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
