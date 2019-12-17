use std::process::Command;
use std::io::{self, Write};

extern crate clap;
use clap::{Arg, App, SubCommand};

use denv::{ItemGraph, Item};
use denv::build::Installer;

fn main() {
    let matches = App::new("Denv")
                    .version("0.1.0")
                    .author("Hugh Yang. <suidar@foxmail.com>")
                    .about("Development environment builder")
                    .subcommand(SubCommand::with_name("install")
                        .about("Install items")
                        .arg(Arg::with_name("item")
                            .help("The item to install")
                            .required(true)
                            .index(1)))
                    .subcommand(SubCommand::with_name("debug"))
                    .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(mut graph) = ItemGraph::from_yaml_file("items.yml") {
            let mut items: Vec<String> = Vec::new();
            if let Some(item_name) = matches.value_of("item") {
                match graph.travel_from(item_name, &mut |item: &mut Item| items.push(item.name.clone())) {
                    Ok(()) => {
                        let installer = Installer::new(&mut items);
                        installer.install();
                    }
                    _ => println!("Wrong config file!"),
                }
            }
        }
    }

    if let Some(_) = matches.subcommand_matches("debug") {
        let output = Command::new("ls")
                        .output()
                        .expect("No way");
        println!("Status: {};", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stdout().write_all(&output.stderr).unwrap();
        println!("Success: {}.", output.status.success());
    }
}