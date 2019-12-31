extern crate clap;
extern crate sqlite;

use std::path::Path;
use graphic_info::{GraphicInfoFile, Database};
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .arg(Arg::with_name("GraphicInfo.bin")
                            .help("The path of GraphicInfo.bin")
                            .required(true))
                    .subcommand(SubCommand::with_name("info")
                            .about("Show the information of graphic info file."))
                    .subcommand(SubCommand::with_name("dump")
                            .about("Dump all of graphic info into sqlite file.")
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("The filename for dump output.")
                                .default_value("dump.sqlite")))
                    .get_matches();

    let path = Path::new(matches.value_of("GraphicInfo.bin").unwrap());
    let mut file = GraphicInfoFile::new(path)?;

    match matches.subcommand() {
        ("dump", Some(sub_matches)) => {
            let database = Database::new(sub_matches.value_of("output").unwrap())?;
            database.migrate().unwrap();

            let _ = file.dump_into(&database);
        }
        ("info", _) | _ => {
            file.show_info();
        },
    }

    Ok(())
}
