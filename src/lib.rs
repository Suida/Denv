use std::iter::FromIterator;
use std::collections::HashMap;
use std::error;
use std::fmt;

use serde_yaml;

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

pub struct ItemGraph {
    items: Vec<Item>,
    graph: Vec<Vec<bool>>,
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

        Some(ItemGraph {items, graph})
    }

    pub fn from_hashmap(map: &HashMap<String, Vec<String>>) -> Option<ItemGraph> {
        // Collect all keys of the map
        let items: Vec<Item> = map.iter()
                                    .map(|(x, _)| {Item::new(x)})
                                    .collect();
        let len = items.len();  // Cache the length

        // Initialize 2-dimensional vector
        let mut graph: Vec<Vec<bool>> = Vec::with_capacity(len);
        for i in 0..len {
            let mut line = Vec::with_capacity(len);

            for _ in 0..len {
                line.push(false);
            }

            // This will always be true
            if let Some(vec) = map.get(&items[i].name) {
                for each in vec.iter() {
                    if let Some(idx) = items.iter().position(|item| item.name == *each) {
                        line[idx] = true;
                    }
                }
            }

            graph.push(line);
        }

        Some(ItemGraph {items, graph})
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
                return ItemGraph::from_hashmap(&map);
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

        self._travel(idx, f)
    }

    pub fn _travel<F>(&mut self, i: usize, f: &mut F) -> Result<(), CycleGraphError>
        where F: FnMut(&mut Item)
    {
        let item = &mut self.items[i];

        if item.backtraced == true {
            return Ok(());
        }

        if item.discovered == true {
            return Err(CycleGraphError);
        }

        item.discovered = true;

        let indice: Vec<usize> = self.graph[i]
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, val)| **val)
                                        .map(|(idx, _)| idx)
                                        .collect();

        for j in indice.iter() {
            if let Err(e) = self._travel(*j, f) {
                return Err(e);
            }
        }

        f(&mut self.items[i]);

        self.items[i].backtraced = true;

        Ok(())
    }
}


pub struct Installer {
    stack: Vec<String>,
}

impl Installer {
    pub fn new(stack: &mut Vec<String>) -> Installer {
        let stack = Vec::from_iter(stack.iter().map(|s| String::from(s)));
        Installer {stack}
    }

    pub fn install(&self) {
        for item in self.stack.iter() {
            println!("Installing {}...", item);
            println!("Installed {}!", item);
        }
    }
}