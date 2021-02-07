extern crate anyhow;
extern crate clap;
#[macro_use] extern crate nom;
#[macro_use] extern crate nom_trace;
extern crate serde;
extern crate serde_derive;
extern crate encoding_rs;
extern crate byteorder;

pub mod dsarc;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use anyhow::Result;
use clap::{Arg, App, SubCommand, ArgMatches, crate_version, crate_authors};

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("dsarc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Unpack NIS DSARC FL (Disgaea 6)")
        .subcommand(SubCommand::with_name("unpack")
                    .about("Unpack a DSARC file")
                    .arg(Arg::with_name("output-dir")
                         .short("o")
                         .long("output-dir")
                         .help("output directory")
                         .takes_value(true)
                         .value_name("DIR"))
                    .arg(Arg::with_name("raw")
                         .short("r")
                         .long("raw")
                         .help("don't automatically dissassemble files"))
                    .arg(Arg::with_name("FILE")
                         .required(true)
                         .help(".dat file")
                         .index(1))
        )
}

fn cmd_unpack(sub_matches: &ArgMatches) -> Result<()> {
    let input_file = Path::new(sub_matches.value_of("FILE").unwrap());
    let output_dir = match sub_matches.value_of("output-dir") {
        Some(dir) => Path::new(dir),
        None => input_file.parent().unwrap()
    };

    fs::create_dir_all(output_dir)?;
    let arc = dsarc::load(&input_file)?;

    for (i, entry) in arc.header.entries.iter().enumerate() {
        let data = &arc.data[i];
        let output_file = output_dir.join(&entry.filename);
        let mut file = File::create(&output_file)?;
        file.write_all(data)?;
    }

    println!("Wrote {} files to {:?}.", arc.header.entries.len(), output_dir);
    Ok(())
}

fn main() -> Result<()> {
    let matches = get_app().get_matches();

    match matches.subcommand() {
        ("unpack", Some(sub_matches)) => cmd_unpack(&sub_matches)?,
        _ => get_app().print_long_help()?
    }

    Ok(())
}
