use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    get_line();
    let nums_a: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let nums_b: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut hash_b: HashMap<i32, i32> = HashMap::new();
    for val in nums_b.into_iter() {
        let x = hash_b.entry(val).or_insert(0);
        *x += 1;
    }

    let mut num_pairs = 0;
    for a in &nums_a {
        match hash_b.entry(*a) {
            Entry::Occupied(mut entry) => {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.get_mut() -= 1;
                }
                num_pairs += 1;
            },
            Entry::Vacant(_) => {},
        }
    }

    if num_pairs == nums_a.len() {
        println!("{}", nums_a.len() - 1);
    } else {
        println!("{}", num_pairs + 1);
    }
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
