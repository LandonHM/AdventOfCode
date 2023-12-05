use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let mut min = 1000000000;
    let file_in = read_to_string("sample_input.txt").unwrap();
    //let file_in = read_to_string("input.txt").unwrap();
    let mut iter = file_in.lines().into_iter();
    let seeds: Vec<usize> = iter.next().expect("first line should exist").trim_start_matches("seeds: ").split(" ").map(|x| x.parse::<usize>().expect("should be number")).collect();
    let mut maps: Vec<Vec<Vec<usize>>> = vec![];
    let mut i = 0;
    // Put maps into vectors
    for line in iter {
        if line == "" {
            continue;
        }
        if line.as_bytes()[0].is_ascii_digit() {
            maps[i-1].push(line.split(" ").map(|x| x.parse::<usize>().expect("Should be numbers in maps")).collect::<Vec<usize>>());
        } else {
            i += 1;
            maps.push(vec![]);
        }
    }

    // sort each map to make finding easier
    for map in maps.iter_mut() {
        map.sort_by(|x, y| x[1].cmp(&y[1]));
    }

    // Find each seed and get lowest
    let mut current;
    let mut idx;
    let seed = seeds[0];
    for chunk in seeds.chunks_exact(2) {
        println!(" at {chunk:?}");
        for i in chunk[0]..chunk[0]+chunk[1] {
            current = i;
            for map in maps.iter() {
                idx = 0;
                loop {
                    // if is in the range, then remap
                    let mapping = map.get(idx);
                    match mapping {
                        Some(m) => {
                            if current >= m[1] && current < m[1] + m[2] {
                                if m[1] < m[0]{
                                    current += m[0] - m[1];
                                }
                                else {
                                    current -= m[1] - m[0];
                                }
                                break;
                            }
                        },
                        None => {
                            break;
                        },
                    }
                    idx += 1;
                }
            }
            min = std::cmp::min(min,current);
        }
    }
    //dbg!(maps);
    println!("{min}");
}
