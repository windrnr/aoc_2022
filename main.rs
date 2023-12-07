use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut count = 0;

    for pairs in &lines {
        if let Some((first, second)) = pairs.split_once(',') {
            if let Some((start, finish)) = first.split_once('-') {
                let s1: u32 = start.parse().unwrap();
                let f1: u32 = finish.parse().unwrap();
                if let Some((start, finish)) = second.split_once('-') {
                    let s2: u32 = start.parse().unwrap();
                    let f2: u32 = finish.parse().unwrap();

                    if !((f1 < f2 && s1 < s2 && f1 < s2) || (f1 > f2 && s1 > s2 && s1 > f2)) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{count}");
}
