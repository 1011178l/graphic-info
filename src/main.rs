extern crate clap;

use std::fs::File;
use std::io::Read;
use graphic_info::GraphicInfo;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .arg(Arg::with_name("GraphicInfo.bin")
                            .help("The path of GraphicInfo.bin")
                            .required(true))
                    .get_matches();

    let file = GraphicInfoFile::new(matches.value_of("GraphicInfo.bin").unwrap())?;
    let collection: Vec<GraphicInfo> = file.collect();

    println!("{}", collection.len());

    Ok(())
}

struct GraphicInfoFile {
    file: File,
}

impl GraphicInfoFile {
    fn new (path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(&path)?;

        if file.metadata()?.len() % 40 != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input file size."));
        }

        Ok(Self{file})
    }
}

impl Iterator for GraphicInfoFile {
    type Item = GraphicInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 40];

        match self.file.read_exact(&mut buf) {
            Ok(_) => return Some(GraphicInfo::new(&buf).unwrap()),
            Err(_) => return None,
        }
    }
}