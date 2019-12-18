use std::iter::FromIterator;
use std::process::Command;

mod build;

use self::build::*;

pub struct Installer {
    items: Vec<String>,
    proxy: String,
}

impl Installer {
    pub fn new(items: &mut Vec<String>, proxy: Option<&str>) -> Installer {
        let items = Vec::from_iter(items.iter().map(|s| String::from(s)));
        let proxy = match proxy {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        
        Installer {items, proxy}
    }

    pub fn set_proxy_env(&self) {
        assert!(self.proxy.starts_with("http://"));

        let _output = Command::new("export")
                        .arg(format!("http_proxy={}", &self.proxy))
                        .output()
                        .expect("HTTP proxy set failed.");

        let _output = Command::new("export")
                        .arg(format!("https_proxy={}", &self.proxy))
                        .output()
                        .expect("HTTP proxy set failed.");

        println!("Using proxy server: {}", &self.proxy);
    }

    pub fn install(&self) {
        for item in self.items.iter() {
            println!("Installing {}...", item);

            let output = match item.as_str() {
                "apt" => change_apt_source(),
                s if s.ends_with("<apt-install>") => apt_install(s),
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
