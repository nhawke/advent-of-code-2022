use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect(&format!("no file at path {}", file_path));

    let max_cals: u32 = contents
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|y| str::parse::<u32>(y).unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{}", max_cals);
}
