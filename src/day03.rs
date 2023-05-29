use std::collections::HashSet;

pub fn run(input: &str) {
    println!("\n\nDay 03");
    let split_compartments: &[&[u8]] = input
        .lines()
        .map(|rucksack| rucksack.as_bytes().split_at(rucksack.len() / 2))
        .collect();

    let _total_priority = 0;
    for bag in split_compartments {
        println!("{:?}", bag);
    }
}

fn _translate_priority(letter: char) -> u32 {
    let mut priority = letter as u32;
    match priority {
        65..=90 => priority -= 38,
        97..=122 => priority -= 96,
        _ => {}
    }
    priority
}
