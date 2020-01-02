extern crate clap;
extern crate sqlite;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use graphic_info::{GraphicInfo, GraphicInfoFile, Database};
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
            let database = Database::open(sub_matches.value_of("input").unwrap())?;
            let mut output = File::create(Path::new(sub_matches.value_of("output").unwrap()))?;

            let mut cursor = database.connection.prepare("SELECT * FROM graphic_info")?.cursor();

            while let Some(row) = cursor.next()? {
                let graphic_info = GraphicInfo {
                    id: row[1].as_integer().unwrap() as u32,
                    address: row[2].as_integer().unwrap() as u32,
                    length: row[3].as_integer().unwrap() as u32,
                    offset_x: row[4].as_integer().unwrap() as i32,
                    offset_y: row[5].as_integer().unwrap() as i32,
                    width: row[6].as_integer().unwrap() as u32,
                    height: row[7].as_integer().unwrap() as u32,
                    tile_east: row[8].as_integer().unwrap() as u8,
                    tile_south: row[9].as_integer().unwrap() as u8,
                    access: row[10].as_integer().unwrap() as u8,
                    unknown: [
                        row[11].as_integer().unwrap() as u8,
                        row[12].as_integer().unwrap() as u8,
                        row[13].as_integer().unwrap() as u8,
                        row[14].as_integer().unwrap() as u8,
                        row[15].as_integer().unwrap() as u8,
                    ],
                    map: row[16].as_integer().unwrap() as u32,
                };

                if bincode::serialize(&graphic_info)? == row[17].as_binary().unwrap() {
                    output.write(row[17].as_binary().unwrap())?;
                } else {
                    let graphic_info = bincode::serialize(&graphic_info).unwrap();
                    database.update(row[0].as_integer().unwrap(), &graphic_info)?;
                    output.write(&graphic_info)?;
                }
            }
        },
        ("dump", Some(sub_matches)) => {
            let mut file = GraphicInfoFile::new(&Path::new(sub_matches.value_of("GraphicInfo.bin").unwrap()))?;
            let database = Database::new(sub_matches.value_of("output").unwrap())?;
            database.migrate()?;

            file.dump_into(&database)?;
        }
        ("info", Some(sub_matches)) => {
            let mut file = GraphicInfoFile::new(&Path::new(sub_matches.value_of("GraphicInfo.bin").unwrap()))?;
            file.show_info();
        },
        _ => {},
    }

    Ok(())
}
