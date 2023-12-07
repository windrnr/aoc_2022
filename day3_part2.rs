
use std::{fs, collections::HashMap};

fn main() { 
    let mut values:HashMap<char, u32> = HashMap::new();
    let mut count = 1;

    for i in 'a'..='z' {
        values.entry(i).or_insert(count);
        count += 1;
    }
    for i in 'A'..='Z' {
        values.entry(i).or_insert(count);
        count += 1;
    }


    let file = fs::read_to_string("input").unwrap();
    let lines:Vec<&str> = file.lines().collect();
    let mut sum = 0;

    let mut iter = lines.iter();

    while iter.len() > 0 {
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        for i in first.chars() {
            if second.contains(i) && third.contains(i) {
                sum += values.get(&i).unwrap();
                break
            }
        }
    }
    println!("{}", sum);
}

