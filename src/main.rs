extern crate clap;

use graphic_info::{GraphicInfo, GraphicInfoFile};
use clap::{Arg, App, SubCommand};

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .arg(Arg::with_name("GraphicInfo.bin")
                            .help("The path of GraphicInfo.bin")
                            .required(true))
                    .subcommand(SubCommand::with_name("info")
                            .about("Show the information of graphic info file."))
                    .get_matches();


    let mut file = GraphicInfoFile::new(matches.value_of("GraphicInfo.bin").unwrap())?;

    match matches.subcommand() {
        ("info", _) | _ => {
            file.show_info();
        },
    }

    Ok(())
}
