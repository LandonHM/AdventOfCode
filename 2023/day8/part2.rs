use std::fs::read_to_string;
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {

    //let in_file = read_to_string("sample_input_p2.txt").unwrap();
    let in_file = read_to_string("input.txt").unwrap();
    let mut input = in_file.lines();
    let directions = input.next().expect("have first line");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    input.next(); // get rid of empty newline

    let mut currents: Vec<&str> = vec![];
    for line in input {
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        if key.ends_with("A") {
            currents.push(key);
        }
        // hardcoded split input is consistent
        let val: Vec<&str> = (&split.next().unwrap()[1..9]).split(", ").collect();
        map.insert(key, (val[0], val[1]));
    }

    // zzz should always be reachable
    let mut counts: Vec<usize> = vec![];
    for current in currents.into_iter() {
        let mut count = 0;
        let mut current = current;
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
                if current.ends_with('Z') {
                    counts.push(count);
                    break 'outer;
                }
            }
        }
    }
    let lcm = get_lcm(counts);
    println!("Count : {lcm}");
}

fn get_lcm(nums: Vec<usize>) -> usize {
    let mut lcm = 1;
    for n in nums {
        lcm = lcm_n(lcm, n);
    }
    lcm
}

// all code below was stolen
fn lcm_n(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
