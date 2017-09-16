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

    let mut hash_b = HashMap::new();
    for val in nums_b.iter() {
        let x = hash_b.entry(val).or_insert(0);
        *x += 1;
    }

    // println!("nums_a: {:?}", nums_a);
    // println!("nums_b: {:?}", nums_b);
    // println!("hash_b: {:?}", hash_b);

    let mut num_pairs = 0;
    for a in &nums_a {
        match hash_b.entry(a) {
            Entry::Vacant(_) => {},
            Entry::Occupied(ref entry) if *entry.get() == 0 => {},
            Entry::Occupied(mut entry) => {
                num_pairs += 1;
                *entry.get_mut() -= 1;
            },
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
