use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut contains = 0;
    let mut overlaps = 0;

    for line in input.lines() {
        let (a1, a2) = line.split_once(",").unwrap();
        let a1 = Assignment::parse(a1);
        let a2 = Assignment::parse(a2);

        if a1.contains(&a2) || a2.contains(&a1) {
            contains += 1;
        }
        if a1.overlaps(&a2) {
            overlaps += 1
        }
    }

    println!("part a: {}", contains);
    println!("part b: {}", overlaps);
}

struct Assignment {
    min: u32,
    max: u32,
}

impl Assignment {
    fn parse(assignment: &str) -> Assignment {
        let (min, max) = assignment
            .split_once("-")
            .unwrap_or_else(|| panic!("invalid assignment {}", assignment));

        let min: u32 = min.parse::<u32>().unwrap();
        let max: u32 = max.parse::<u32>().unwrap();
        Assignment { min, max }
    }

    fn contains_point(&self, point: u32) -> bool {
        self.min <= point && point <= self.max
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains_point(other.min)
            || self.contains_point(other.max)
            || other.contains_point(self.min)
            || other.contains_point(self.max)
    }
}
