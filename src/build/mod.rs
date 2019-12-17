use std::iter::FromIterator;

mod build;

use self::build::*;

pub struct Installer {
    items: Vec<String>,
}

impl Installer {
    pub fn new(items: &mut Vec<String>) -> Installer {
        let items = Vec::from_iter(items.iter().map(|s| String::from(s)));
        Installer {items}
    }

    pub fn install(&self) {
        for item in self.items.iter() {
            println!("Installing {}...", item);

            let output = match item.as_str() {
                "apt" => apt(),
                "git" => git(),
                "curl" => curl(),
                _ => Ok(()),
            };

            if let Err(output) = output {
                println!("{}", output);
            } else {
                println!("Installed {}!", item);
            }
        }
    }
}
