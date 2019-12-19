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

/// The struct to store item to install as nodes of a graph, that's why
/// `discovered` and `backtraced` are add as attributes.
pub struct Item {
    pub name: String,
    discovered: bool,
    backtraced: bool,
}

impl Item {
    /// Initializer for `Item` struct
    pub fn new(name: &str) -> Item {
        Item {
            name: String::from(name),
            discovered: false,
            backtraced: false,
        }
    }

    /// Before each travel, this method should be called to set it as the
    /// original status.
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

use TravelMode::{Post, Pre};

/// The directed acyclic graph structure to store the dependency relationship
/// among the items to install. With several `from` methods, it can be easily
/// generated with a few amount of configure.
/// 
/// **NOTE**: Each individual item should have an identical name.
/// 
/// # Example
/// 
/// ```rust
/// # use denv::ItemGraph;
/// let mut graph = ItemGraph::from_yaml_file("/path/to/yaml");
/// ```
pub struct ItemGraph {
    items: Vec<Item>,
    graph: Vec<Vec<bool>>,

    pub yaml_file_path: Option<String>,
}

impl ItemGraph {

    /// With a vector with item names as elements, this method creates an item
    /// graph with named items but without any dependency relationship.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use denv::ItemGraph;
    /// let vec = vec!["apt", "git", "rust", "pyenv"];
    /// let mut graph = ItemGraph::from_vec(vec)
    /// # ;
    /// ```
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

    /// The real productive method to create `ItemGraph`.
    /// 
    /// The keys of `map: &HashMap<String, Vec<String>>` are items and their
    /// values are the items which they depend on and are also the keys.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use denv::ItemGraph;
    /// let mut map: &HashMap<String, Vec<String>> = HashMap::new();
    /// 
    /// map.insert(
    ///     "curl",
    ///     vec![]
    /// );
    /// 
    /// map.insert(
    ///     "pyenv",
    ///     vec!["curl"]
    /// )
    /// 
    /// let graph = ItemGraph::from_hashmap(map)
    /// # ;
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

    /// The procedure:
    ///     - Turns standard yaml string to `HashMap<String, Vec<String>>`,
    ///     - call `ItemGraph::from_hashmap` to initialize self.
    pub fn from_yaml(s: &str) -> Option<ItemGraph> {
        let result: Result<HashMap<String, Vec<String>>, serde_yaml::Error> = serde_yaml::from_str(&s); 

        if let Ok(map) = result {
            ItemGraph::from_hashmap(&map)
        } else {
            None
        }
    }

    /// The procedure:
    ///     - Read the yaml file to string,
    ///     - turns standard yaml string to `HashMap<String, Vec<String>>`,
    ///     - call `ItemGraph::from_hashmap` to initialize self.
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

    /// Check if the graph is cyclic graph. If cyclic, return false.
    pub fn validate(&mut self) -> bool {
        match self.travel_from("", &mut |_|{}) {
            Ok(()) => true,
            _ => false,
        }
    }

    /// Travel the graph from the `Item` with name `s` with a 'post' style, and
    /// deal each item with function `f`.
    pub fn travel_from<F>(&mut self, s: &str, f: &mut F) -> Result<(), CycleGraphError>
        where F: FnMut(&mut Item)
    {
        let mut idx = -1;

        for (i, item) in self.items.iter_mut().enumerate() {
            item.clean();
            if item.name == s {
                idx = i as i64;
            }
        }

        // If not found
        if idx < 0 {
            println!("\"{}\" not found", s);
            return Ok(())
        }

        self.travel(idx as usize, f, &Post)
    }

    /// Travel the graph with style specified by `mode` from the No.`i` item in
    /// `self.items` and deal each item with function `f`.
    pub fn travel<F>(
        &mut self,
        i: usize,
        f: &mut F,
        mode: &TravelMode
    ) -> Result<(), CycleGraphError>
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

    /// **How can we figure out whether an item is installed or not?**
    /// 
    /// This is a question easy to introduce. The strategy of this implementation
    /// is to add an extra item with the name "installed". As we talked before, 
    /// in the yaml file or in the hash map which responds the relationships,
    /// every item has its dependency vector with the depended items as its
    /// elements. The elements of the vector of item called installed are those
    /// items which have been installed.
    /// 
    /// *This is a underlying detail which the users should not take care of.*
    /// 
    /// Set the item as installed by add the item name to the dependency items
    /// of item "installed".
    pub fn set_as_installed(&mut self, item_name: &str) -> &mut Self {
        let idx = self.items.iter().position(|x| x.name == item_name ).unwrap();
        self.graph[self.items.len() - 1][idx] = true;
        self
    }

    /// Because there might be some items newly installed, the elements of 
    /// the item call installed in the map should be refreshed and dumped to a
    /// record file, we need a new `HashMap` to respond new status which this
    /// function actually complete.
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

    /// Save the current graph to yaml file.
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

    /// Mark the installed items indicated by `item_names`, save the new graph
    /// to yaml file.
    pub fn update_items(&mut self, item_names: Vec<String>) -> &mut Self {
        for item_name in item_names.iter() {
            self.set_as_installed(&item_name);
        }

        self.to_yaml();
        self
    }

    /// Print the installed items.
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