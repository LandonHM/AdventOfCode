use std::fs::read_to_string;
use std::thread;

#[allow(unused_variables)]
fn main() {
    let mut min = 1000000000;
    //let file_in = read_to_string("sample_input.txt").unwrap();
    let file_in = read_to_string("input.txt").unwrap();
    let mut iter = file_in.lines().into_iter();
    let seeds: Vec<usize> = iter.next().expect("first line should exist").trim_start_matches("seeds: ").split(" ").map(|x| x.parse::<usize>().expect("should be number")).collect();
    let mut maps: Vec<Vec<Vec<isize>>> = vec![];
    let mut i = 0;
    // Put maps into vectors
    for line in iter {
        if line == "" {
            continue;
        }
        if line.as_bytes()[0].is_ascii_digit() {
            maps[i-1].push(line.split(" ").map(|x| x.parse::<isize>().expect("Should be numbers in maps")).collect::<Vec<isize>>());
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
    let mut threads: Vec<_> = vec![];
    let seed = seeds[0];
    for (i,chunk) in seeds.chunks_exact(2).enumerate() {
        let chunk = chunk.clone();
        let maps = maps.clone();
        let start = chunk[0];
        let range = chunk[1];
        println!("starting thread {i} with s = {start} r = {range}");
        threads.push(thread::spawn(move || {
            let mut min = 1000000000;
            for i in start..start+range {
                let mut current = i as isize;
                for map in maps.iter() {
                    let mut idx = 0;
                    loop {
                        // if is in the range, then remap
                        let mapping = map.get(idx);
                        match mapping {
                            Some(m) => {
                                if current >= m[1] && current < m[1] + m[2] {
                                    current += m[0] - m[1];
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
            return min;
        }));
    }
    let mut i = 0;
    for t in threads {
        min = std::cmp::min(t.join().unwrap(),min);
        println!("thread {i} finished");
        i += 1;
    }
    //dbg!(maps);
    println!("{min}");
}
