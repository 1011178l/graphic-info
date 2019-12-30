extern crate clap;

use std::fs::File;
use std::io::Read;
use ::graphic_info::GraphicInfo;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .arg(Arg::with_name("GraphicInfo.bin")
                            .help("The path of GraphicInfo.bin")
                            .required(true))
                    .get_matches();

    let mut file = File::open(matches.value_of("GraphicInfo.bin").unwrap())?;
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
