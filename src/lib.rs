use std::fs::OpenOptions;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::Write;

use serde_yaml;

pub mod build;

#[derive(Debug, Clone)]
pub struct CycleGraphError;

impl fmt::Display for CycleGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cyclic graph is not allowed!")
    }
}

impl error::Error for CycleGraphError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct Item {
    pub name: String,
    discovered: bool,
    backtraced: bool,
}

impl Item {
    pub fn new(name: &str) -> Item {
        Item {
            name: String::from(name),
            discovered: false,
            backtraced: false,
        }
    }

    pub fn clean(&mut self) -> &Item{
        self.discovered = false;
        self.backtraced = false;
        self
    }
}

pub enum TravelMode {
    Pre,
    In,
    Post,
}

use TravelMode::*;

pub struct ItemGraph {
    items: Vec<Item>,
    graph: Vec<Vec<bool>>,

    pub yaml_file_path: Option<String>,
}

impl ItemGraph {
    pub fn from_vec(vec: &Vec<&str>) -> Option<ItemGraph> {
        let len = vec.len();

        let mut items: Vec<Item> = Vec::with_capacity(len);
        items.extend(vec.iter().map(|s| Item::new(s)));

        let mut graph: Vec<Vec<bool>> = Vec::with_capacity(len);
        for i in 0..len {
            let mut line = Vec::with_capacity(len);
            for _ in 0..len {
                line.push(false);
            }
            graph[i] = line;
        }

        Some(ItemGraph {items, graph, yaml_file_path: None})
    }

    pub fn from_hashmap(map: &HashMap<String, Vec<String>>) -> Option<ItemGraph> {
        // Collect all keys of the map
        let mut items: Vec<Item> = map.iter()
                                    .filter(|(x, _)| *x != "installed" )
                                    .map(|(x, _)| {Item::new(x)})
                                    .collect();
        items.push(Item::new("installed"));
        let len = items.len();  // Cache the length

        // Initialize 2-dimensional vector
        let mut graph: Vec<Vec<bool>> = Vec::with_capacity(len);
        for item in items.iter() {
            let mut line = vec![false; len+1];

            // This will always be true
            if let Some(vec) = map.get(&item.name) {
                for each in vec.iter() {
                    if let Some(idx) = items.iter().position(|item| item.name == *each) {
                        line[idx] = true;
                    }
                }
            }

            graph.push(line);
        }

        Some(ItemGraph {items, graph, yaml_file_path: None})
    }

    pub fn from_yaml(s: &str) -> Option<ItemGraph> {
        let result: Result<HashMap<String, Vec<String>>, serde_yaml::Error> = serde_yaml::from_str(&s); 

        if let Ok(map) = result {
            ItemGraph::from_hashmap(&map)
        } else {
            None
        }
    }

    pub fn from_yaml_file(s: &str) -> Option<ItemGraph> {
        if let Ok(f) = std::fs::File::open(&s) {
            let result: Result<HashMap<String, Vec<String>>, serde_yaml::Error> = serde_yaml::from_reader(f);

            if let Ok(map) = result {
                if let Some(mut ret) = ItemGraph::from_hashmap(&map) {
                    ret.yaml_file_path = Some(s.to_string());
                    return Some(ret)
                }
            }
        }
        
        None
    }

    pub fn validate(&mut self) -> bool {
        match self.travel_from("", &mut |_|{}) {
            Ok(()) => true,
            _ => false,
        }
    }

    pub fn travel_from<F>(&mut self, s: &str, f: &mut F) -> Result<(), CycleGraphError>
        where F: FnMut(&mut Item)
    {
        let mut idx = 0;

        for (i, item) in self.items.iter_mut().enumerate() {
            item.clean();
            if item.name == s {
                idx = i;
            }
        }

        self.travel(idx, f, &Post)
    }

    pub fn travel<F>(&mut self, i: usize, f: &mut F, mode: &TravelMode) -> Result<(), CycleGraphError>
        where F: FnMut(&mut Item)
    {
        let item = &mut self.items[i];

        // If backtraced or has been installed
        if item.backtraced || self.graph.last().unwrap()[i] {
            return Ok(());
        }

        if item.discovered {
            return Err(CycleGraphError);
        }

        item.discovered = true;

        if let Pre = mode {
            f(&mut self.items[i]);
        }

        let indice: Vec<usize> = self.graph[i]
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, val)| **val)
                                        .map(|(idx, _)| idx)
                                        .collect();

        for j in indice.iter() {
            if let Err(e) = self.travel(*j, f, &mode) {
                return Err(e);
            }
        }

        if let Post = mode {
            f(&mut self.items[i]);
        }

        self.items[i].backtraced = true;

        Ok(())
    }

    pub fn set_as_installed(&mut self, item_name: &str) -> &mut Self {
        let idx = self.items.iter().position(|x| x.name == item_name ).unwrap();
        self.graph[self.items.len() - 1][idx] = true;
        self
    }

    pub fn to_hash_map(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        for i in 0..self.items.len() {
            map.insert(
                self.items[i].name.to_string(),
                self.graph[i].iter()
                                .enumerate()
                                .filter(|(_, x)| **x )
                                .map(|(i, _)| self.items[i].name.to_string() )
                                .collect()
            );
        }

        map
    }

    pub fn to_yaml(&self) {
        if let Some(s) = &self.yaml_file_path {
            match OpenOptions::new().create(true).write(true).open(s) {
                Ok(mut file) => {
                    match serde_yaml::to_string(&self.to_hash_map()) {
                        Ok(s) => {
                            file.write_all(s.as_bytes()).unwrap();
                        },
                        Err(e) => println!("Permenent failed: {}", e),
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }

    pub fn update_items(&mut self, item_names: Vec<String>) -> &mut Self {
        for item_name in item_names.iter() {
            self.set_as_installed(&item_name);
        }

        self.to_yaml();
        self
    }

    pub fn installed(&self) {
        println!("{:?}", self.graph.last()
                                    .unwrap()
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, v)| **v )
                                    .map(|(i, _)| self.items[i].name.to_string())
                                    .collect::<Vec<String>>()
        )
    }
}