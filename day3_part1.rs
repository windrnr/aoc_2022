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
    
    for line in &lines {
        let (first, second) = line.split_at(line.len()/2);
        for i in first.chars() {
            if second.contains(i) {
                sum += values.get(&i).unwrap();
                break
            }
        }
    }

    println!("{}", sum);
}
