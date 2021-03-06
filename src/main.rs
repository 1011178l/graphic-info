extern crate clap;
extern crate sqlite;

use std::path::Path;
use graphic_info::{
    data_structure::GraphicInfoFile,
    storage::Sqlite,
};
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .subcommand(SubCommand::with_name("info")
                            .about("Show the information of graphic info file.")
                            .arg(Arg::with_name("GraphicInfo.bin")
                                .help("The path of GraphicInfo.bin")
                                .required(true)))
                    .subcommand(SubCommand::with_name("dump")
                            .about("Dump all of graphic info into sqlite file.")
                            .arg(Arg::with_name("GraphicInfo.bin")
                                .help("The path of GraphicInfo.bin")
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .help("The file name for dump output")
                                .default_value("dump.sqlite")))
                    .subcommand(SubCommand::with_name("build")
                            .about("Build GraphicInfo.bin from sqlite.")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .long("input")
                                .value_name("GraphicInfo.sqlite")
                                .help("The path of sqlite file.")
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .value_name("GraphicInfo.bin")
                                .help("The output of GraphicInfo.bin")
                                .default_value("GraphicInfo.bin")))
                    .get_matches();

    match matches.subcommand() {
        ("build", Some(sub_matches)) => {
            let mut file = GraphicInfoFile::new(Path::new(sub_matches.value_of("output").unwrap()))?;
            let database = Sqlite::open(sub_matches.value_of("input").unwrap())?;

            file.build_from(&database)?;
        },
        ("dump", Some(sub_matches)) => {
            let mut file = GraphicInfoFile::open(&Path::new(sub_matches.value_of("GraphicInfo.bin").unwrap()))?;
            let database = Sqlite::new(sub_matches.value_of("output").unwrap())?;
            database.migrate()?;

            file.dump_into(&database)?;
        }
        ("info", Some(sub_matches)) => {
            let mut file = GraphicInfoFile::open(&Path::new(sub_matches.value_of("GraphicInfo.bin").unwrap()))?;
            file.show_info();
        },
        _ => {},
    }

    Ok(())
}
