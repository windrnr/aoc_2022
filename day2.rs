use std::fs;

fn main() { 
    let file = fs::read_to_string("input").unwrap();
    let lines:Vec<&str> = file.lines().collect();
    let mut puntaje = 0;
    for i in lines {
        let pair:Vec<&str> = i.split(' ').collect();
        let mut p = pair.iter();
        let first = p.next().unwrap();
        let second = p.next().unwrap();

        match (*first, *second) {
            ("A", "Y") => { puntaje += 1 + 3}
            ("A", "X") => { puntaje += 3 + 0}
            ("A", "Z") => { puntaje += 2 + 6}
            ("B", "Y") => { puntaje += 2 + 3}
            ("B", "X") => { puntaje += 1 + 0}
            ("B", "Z") => { puntaje += 3 + 6}
            ("C", "Y") => { puntaje += 3 + 3}
            ("C", "X") => { puntaje += 2 + 0}
            ("C", "Z") => { puntaje += 1 + 6}
            (_,_) => ()
        }
    }

    println!("{}", puntaje);
}


