mod test;

use std::collections::HashMap;

pub struct Trie(Vertex);

impl Trie {
    pub fn new(src: &Vec<&str>) -> Self {
        let mut root = Vertex::new(false);
        for str in src {
            if str.is_empty() {
                panic!("Blank string not supported");
            }
            root.add(str, 0);
        }
        Trie(root)
    }

    pub fn find(&self, str: &str) -> bool {
        Self::find_base(str, 0, &self.0)
    }

    pub fn add(&mut self, str: &str) {
        self.0.add(str, 0);
    }

    fn find_base(src: &str, index: usize, vertex: &Vertex) -> bool {
        if index == src.len() {
            return vertex.is_terminal;
        }
        let ch = src.chars().nth(index).unwrap();
        match vertex.edges.get(&ch) {
            Some(next_vertex) => Self::find_base(src, index + 1, next_vertex),
            None => false,
        }
    }
}

struct Vertex {
    edges: HashMap<char, Vertex>,
    is_terminal: bool,
}

impl Vertex {
    fn new(is_terminal: bool) -> Self {
        Vertex {
            is_terminal,
            edges: HashMap::new(),
        }
    }
}

impl Vertex {
    fn add(&mut self, src: &str, index: usize) {
        let ch = src.chars().nth(index).unwrap();
        match self.edges.get_mut(&ch) {
            None => {
                let is_terminal = src.len() - 1 == index;
                let mut new_vertex = Vertex::new(is_terminal);

                if !is_terminal {
                    new_vertex.add(src, index + 1);
                }
                self.edges.insert(ch, new_vertex);
            }
            Some(v) => v.add(src, index + 1),
        }
    }
}
