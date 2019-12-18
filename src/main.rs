use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

extern crate clap;
use clap::{Arg, App, SubCommand};

use denv::{ItemGraph, Item};
use denv::build::Installer;

const ITEMS_YAML: &'static str = "items.yml";

const RECORDS_YAML: &'static str = "records.yml" ;

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
        let yaml_path = if Path::new(RECORDS_YAML).exists() {
            RECORDS_YAML
        } else {
            ITEMS_YAML
        };

        if let Some(mut graph) = ItemGraph::from_yaml_file(yaml_path) {
            graph.yaml_file_path = Some("records.yml".to_string());

            let mut items: Vec<String> = Vec::new();

            if let Some(item_name) = matches.value_of("item") {
                match graph.travel_from(item_name, &mut |item: &mut Item| items.push(item.name.clone())) {
                    Ok(()) => {
                        Installer::new(&mut items).install();

                        graph.update_items(items);
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