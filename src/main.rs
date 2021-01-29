mod diff;

use clap::{Arg, App};
use crate::diff::{Resource};
// use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("DiffOps")
        .version("0.1")
        .author("Selçuk Öztürk <selcukahmed@gmail.com>")
        .about("A tool that compares directories and files")

        .arg(Arg::new("reference")
            .long("reference")
            .about("Reference folder")
            .multiple(false)
            .takes_value(true)
            .required(false))

        .arg(Arg::new("target")
            .long("target")
            .about("Target folder")
            .multiple(false)
            .takes_value(true)
            .required(false))

        .arg(Arg::new("list")
            .long("list")
            .short('l')
            .about("Prints the reference and the target folder")
            .multiple(false)
            .required(false))

        .get_matches();


    let mut resources = Resource::get()?;

    // let mut diff = Diff { reference: String::new(), target: String::new() };
    //
    if let Some(path) = matches.value_of("reference") {
        resources.set_reference(path)?;
        resources.put()?;
    }

    if let Some(path) = matches.value_of("target") {
        resources.set_target(path)?;
        resources.put()?;
    }

    if matches.is_present("list") {
        println!("Reference: {}", resources.reference());
        println!("Target: {}", resources.target());
    }

    // let current_dir = env::current_dir().unwrap();
    // diff::traverse_folders(&current_dir);

    Ok(())
}

