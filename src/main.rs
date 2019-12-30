extern crate clap;

use graphic_info::{GraphicInfo, GraphicInfoFile};
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
