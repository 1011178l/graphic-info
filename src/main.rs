use ::graphic_info::GraphicInfo;

fn main() {
    let bytes = [0; 40];
    let graphic_info = GraphicInfo::new(&bytes);

    println!("{:?}", graphic_info);
}
