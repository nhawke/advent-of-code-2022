use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total: u32 = 0;

    'lines: for rucksack in input.lines() {
        let compartments = rucksack.split_at(rucksack.len() / 2);

        for item in compartments.0.chars() {
            if compartments.1.contains(item) {
                total += item.to_lowercase().last().unwrap() as u32 - 'a' as u32 + 1;
                if item.is_uppercase() {
                    total += 26;
                }
                continue 'lines;
            }
        }
        panic!(
            "No duplicate items found. Rucksack: {}. Compartments {:?}",
            rucksack, compartments
        );
    }

    println!("{}", total);
}
