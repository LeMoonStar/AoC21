use super::{Day, DayImpl};
use std::collections::HashMap;

const CURRENT_DAY: u8 = 12;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    connections: Vec<String>,
    big_cave: bool,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            connections: Vec::new(),
            big_cave: name.chars().nth(0).unwrap().is_uppercase(),
        }
    }

    fn connect(&mut self, other: &str) {
        self.connections.push(other.to_owned());
    }

    fn count_paths(
        &self,
        dest: &str,
        nodes: &HashMap<String, Node>,
        ignore: &mut Vec<String>,
        //path: &mut Vec<String>,
    ) -> u64 {
        //path.push(self.name.clone());
        if dest == self.name {
            //path.pop();
            return 1;
        }

        if !self.big_cave {
            ignore.push(self.name.clone());
        }
        let mut paths = 0;

        self.connections.iter().for_each(|v| {
            if !ignore.contains(v) {
                paths += nodes
                    .get(v)
                    .unwrap()
                    .count_paths(dest, nodes, ignore /*, path*/);
            }
        });
        if !self.big_cave {
            if ignore.pop().unwrap_or("".to_owned()) != self.name {
                panic!("error");
            }
        }
        //path.pop();

        paths
    }

    fn count_paths_visit_twice(
        &self,
        dest: &str,
        nodes: &HashMap<String, Node>,
        ignore: &mut Vec<String>,
        allow_twice: (String, u8),
        //path: &mut Vec<String>,
    ) -> u64 {
        //path.push(self.name.clone());
        if dest == self.name {
            if allow_twice.0 == "" {
                //println!("Path: {:?}", path);
                //path.pop();
                return 1;
            } else if allow_twice.1 == 2 {
                //println!("Path: {:?}", path);
                //path.pop();
                return 1;
            } else {
                return 0;
            }
        }
        let mut paths = 0;

        if allow_twice.0 == "" && !self.big_cave {
            self.connections.iter().for_each(|v| {
                if !ignore.contains(v) {
                    paths += nodes.get(v).unwrap().count_paths_visit_twice(
                        dest,
                        nodes,
                        ignore,
                        (self.name.clone(), 1),
                        //path,
                    );
                }
            });
        }

        let mut allow_twice = allow_twice;
        if allow_twice.0 == self.name {
            allow_twice.1 += 1;
        }

        if !self.big_cave {
            ignore.push(self.name.clone());
        }
        self.connections.iter().for_each(|v| {
            if !ignore.contains(v) {
                paths += nodes.get(v).unwrap().count_paths_visit_twice(
                    dest,
                    nodes,
                    ignore,
                    allow_twice.clone(),
                    //path,
                );
            }
        });
        if !self.big_cave {
            if ignore.pop().unwrap_or("".to_owned()) != self.name {
                panic!("error");
            }
        }

        //path.pop();
        paths
    }
}

type Data = HashMap<String, Node>;

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test12.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (226, 3509)
    }

    fn init(input: &str) -> (Self, Data) {
        let mut nodes: HashMap<String, Node> = HashMap::new();

        input.lines().for_each(|v| {
            let names: Vec<&str> = v.split('-').collect();
            if !nodes.contains_key(names[0]) {
                nodes.insert(names[0].to_owned(), Node::new(names[0]));
            }
            if !nodes.contains_key(names[1]) {
                nodes.insert(names[1].to_owned(), Node::new(names[1]));
            }

            if let Some(f) = nodes.get_mut(names[0]) {
                f.connect(names[1]);
            }
            if let Some(f) = nodes.get_mut(names[1]) {
                f.connect(names[0]);
            }
        });

        (Self {}, nodes)
    }

    fn one(&self, data: &mut Data) -> u64 {
        data.get("start").unwrap().count_paths(
            "end",
            data,
            &mut Vec::new(), /*&mut Vec::new()*/
        )
    }

    fn two(&self, data: &mut Data) -> u64 {
        data.get("start").unwrap().count_paths_visit_twice(
            "end",
            data,
            &mut vec!["start".to_owned()],
            ("".to_owned(), 0),
            //&mut Vec::new(),
        )
    }
}
