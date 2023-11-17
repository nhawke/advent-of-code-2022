use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut crates = Crates::parse(stacks_input);
    let mut crates2 = Crates::parse(stacks_input);
    for m in moves_input.lines() {
        let m = Move::parse(m);
        crates.do_move(&m);
        crates2.do_multi_move(&m);
    }
    println!("{}", crates.stack_tops());
    println!("{}", crates2.stack_tops());
}

#[derive(Debug)]
struct Crate(char);

#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<Crate>>,
}

impl Crates {
    fn new(size: usize) -> Crates {
        let mut stacks = Vec::with_capacity(size);
        for _ in 0..size {
            stacks.push(Vec::new());
        }
        Crates { stacks }
    }

    fn parse(stacks_input: &str) -> Crates {
        let mut rows = Vec::<Vec<String>>::new();

        for line in stacks_input.lines().rev() {
            let chars: Vec<char> = line.chars().collect();
            let boxes = chars.chunks(4);
            let boxes: Vec<String> = boxes
                .map(|chunk| chunk.iter().filter(|c| c.is_alphanumeric()).collect())
                .collect();

            rows.push(boxes);
        }

        let mut crates = Crates::new(rows[0].len());
        for row in rows.iter().skip(1) {
            for (i, item) in row.iter().enumerate() {
                if item != "" {
                    crates.stacks[i].push(Crate(item.chars().last().unwrap()));
                }
            }
        }
        crates
    }

    fn do_move(&mut self, m: &Move) {
        for _ in 0..m.amount {
            if let Some(item) = self.stacks[m.from - 1].pop() {
                self.stacks[m.to - 1].push(item);
            } else {
                break;
            }
        }
    }

    fn do_multi_move(&mut self, m: &Move) {
        let from_stack = &mut self.stacks[m.from - 1];
        let top = from_stack.split_off(from_stack.len() - m.amount);
        self.stacks[m.to - 1].extend(top);
    }

    fn stack_tops(&self) -> String {
        let mut out = String::new();
        for stack in &self.stacks {
            match stack.last() {
                Some(c) => out.push(c.0),
                _ => {}
            }
        }
        out
    }
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(input: &str) -> Move {
        let nums: Vec<usize> = input
            .split(" ")
            .filter_map(|word| {
                if let Ok(n) = word.parse() {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();

        assert!(nums.len() == 3);
        Move {
            amount: nums[0],
            from: nums[1],
            to: nums[2],
        }
    }
}
