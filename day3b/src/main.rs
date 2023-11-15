use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total: u32 = 0;

    let groups = input.lines().collect::<Vec<_>>();
    let groups = groups.chunks_exact(3);

    'groups: for group in groups {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                println!("Found shared item {} in group {:?}", item, group);
                total += item_priority(item);
                continue 'groups;
            }
        }
        panic!("No shared item in group {:?}", group);
    }
    println!("{}", total);
}

fn item_priority(item: char) -> u32 {
    let mut priority = item.to_lowercase().last().unwrap() as u32 - 'a' as u32 + 1;
    if item.is_uppercase() {
        priority += 26;
    }

    return priority;
}
