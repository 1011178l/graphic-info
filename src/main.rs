use std::env;
use std::process;
use std::fs::File;
use std::io::Read;
use ::graphic_info::GraphicInfo;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./graphic-info [GraphicInfo.bin]");
        process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut buf = [0; 40];
    let mut collection: Vec<GraphicInfo> = vec!();

    loop {
        match file.read_exact(&mut buf) {
            Ok(_) => collection.push(GraphicInfo::new(&buf).unwrap()),
            Err(_) => break,
        }
    }

    println!("{}", collection.len());

    Ok(())
}
