use std::fs::read_to_string;
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    //let in_file = read_to_string("sample_input.txt").unwrap();
    //let in_file = read_to_string("sample_input2.txt").unwrap();
    let in_file = read_to_string("input.txt").unwrap();
    let mut input = in_file.lines();
    let directions = input.next().expect("have first line");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    input.next(); // get rid of empty newline
    let mut first = true;
    for line in input {
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        // hardcoded split input is consistent
        let val: Vec<&str> = (&split.next().unwrap()[1..9]).split(", ").collect();
        map.insert(key, (val[0], val[1]));
    }

    // zzz should always be reachable
    let mut count = 0;
    let mut current = "AAA";
    'outer: loop {
        for c in directions.chars() {
            count += 1;
            match c {
                'R' => {
                    current = map.get(current).expect("Locations should all be valid").1;
                },
                'L' => {
                    current = map.get(current).expect("Locations should all be valid").0;
                },
                _ => { unreachable!("Directions are only L or R"); },
            }
            if current == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Count : {count}");
}
