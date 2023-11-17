use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!(
        "{}",
        unique_window_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)
    );
    println!(
        "{}",
        unique_window_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
    );

    println!("{}", unique_window_index(&input, 4));
    println!("{}", unique_window_index(&input, 14));
}

fn unique_window_index(input: &str, size: usize) -> usize {
    // The index we need is the index AFTER the window.
    let mut end_index: usize = size;

    for sequence in input.as_bytes().windows(size) {
        if unique(sequence) {
            break;
        }

        end_index += 1;
    }
    end_index
}

fn unique(sequence: &[u8]) -> bool {
    let mut seen = HashSet::new();
    for c in sequence.iter() {
        if seen.contains(c) {
            return false;
        }
        seen.insert(c);
    }
    return true;
}
