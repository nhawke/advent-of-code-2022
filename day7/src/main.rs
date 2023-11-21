use std::{
    collections::HashMap,
    fs::{self},
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut files = FileTree::new_directory();
    let mut cwd: Vec<String> = Vec::new();

    // Build the filetree
    for (i, line) in input.lines().enumerate() {
        if line.starts_with('$') {
            let command = line.split(' ').collect::<Vec<_>>();
            match command[1..] {
                ["cd", d] => match d {
                    ".." => {
                        cwd.pop();
                    }
                    "/" => {
                        cwd.clear();
                    }
                    _ => {
                        cwd.push(d.to_string());
                    }
                },
                ["ls"] => {
                    for line in input
                        .lines()
                        .skip(i + 1)
                        .take_while(|l| !l.starts_with("$"))
                    {
                        let (file_size, name) = line.split_once(' ').unwrap();
                        let mut path = cwd.clone();
                        path.push(name.to_string());
                        match file_size.parse::<usize>() {
                            Ok(size) => files.add_file(path, size),
                            Err(_) => files.add_dir(path),
                        }
                    }
                }
                _ => panic!("unexpected command {:?}", command),
            }
        }
    }

    // Calculate sizes.
    const THRESHOLD: usize = 100000;
    let mut dir_sizes: Vec<usize> = Vec::new();
    let mut queue: Vec<&FileTree> = vec![&files];
    loop {
        match queue.pop() {
            Some(fs) => {
                if let FileTree::Directory(d) = fs {
                    dir_sizes.push(fs.size());
                    for (_, item) in d.iter() {
                        queue.push(item);
                    }
                }
            }
            None => break,
        }
    }

    // Calculate puzzle answers.
    let under_threshold_sum: usize = dir_sizes
        .iter()
        .cloned()
        .filter(|x| *x <= THRESHOLD)
        .sum::<usize>();

    println!("sum of dirs under threshold: {}", under_threshold_sum);

    const TOTAL_SIZE: usize = 70000000;
    const SPACE_NEEDED: usize = 30000000;
    let space_available: usize = TOTAL_SIZE - files.size();
    let additional_space_needed: usize = SPACE_NEEDED - space_available;

    let mut closest = usize::MAX;
    for size in dir_sizes {
        if size >= additional_space_needed && size < closest {
            closest = size;
        }
    }

    println!(
        "size of smallest dir that can free up enough space: {}",
        closest
    );
}

#[derive(Debug)]
enum FileTree {
    Directory(HashMap<String, FileTree>),
    File(usize),
}

impl FileTree {
    fn new_directory() -> FileTree {
        FileTree::Directory(HashMap::new())
    }

    fn add_file(&mut self, path: Vec<String>, size: usize) {
        use FileTree::*;

        let mut curr = self;
        for node in path[..path.len() - 1].iter() {
            match curr {
                Directory(d) => curr = d.entry(node.clone()).or_insert(FileTree::new_directory()),
                _ => panic!("this is a file not a path!"),
            }
        }

        match curr {
            Directory(d) => d.insert(path.last().unwrap().clone(), File(size)),
            _ => panic!("can't insert into a non-directory"),
        };
    }

    fn add_dir(&mut self, path: Vec<String>) {
        use FileTree::*;

        let mut curr = self;
        for node in path[..path.len() - 1].iter() {
            match curr {
                Directory(d) => curr = d.entry(node.clone()).or_insert(FileTree::new_directory()),
                _ => panic!("this is a file not a path!"),
            }
        }

        match curr {
            Directory(d) => d.insert(
                path.last().expect("no file name").clone(),
                FileTree::new_directory(),
            ),
            _ => panic!("can't insert into a non-directory"),
        };
    }

    fn size(&self) -> usize {
        match self {
            FileTree::Directory(d) => d.iter().fold(0, |acc, (_, item)| acc + item.size()),
            FileTree::File(s) => *s,
        }
    }
}
