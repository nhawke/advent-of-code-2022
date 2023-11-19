use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut root = Directory::create(String::from(""));
    for line in input.lines() {
        if line.starts_with
    }
}

struct Directory {
    name: String,
    contents: Vec<Node>,
    cached_size: Option<usize>,
}

impl Directory {
    fn create(name: String) -> Directory {
        Directory {
            name: name,
            contents: Vec::new(),
            cached_size: None,
        }
    }
    fn size(&mut self) -> usize {
        if let Some(s) = self.cached_size {
            return s;
        }

        let mut size: usize = 0;
        for node in &mut self.contents {
            match node {
                Node::File(f) => size += f.size,
                Node::Dir(d) => size += d.size(),
            }
        }
        self.cached_size = Some(size);
        size
    }
}

struct File {
    name: String,
    size: usize,
}

enum Node {
    Dir(Directory),
    File(File),
}
