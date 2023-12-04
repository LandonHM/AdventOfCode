use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let mut vec: Vec<usize> = vec![0;1000];
    let mut idx = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
    //for line in read_to_string("sample_input.txt").unwrap().lines() {
        let mut line_split = line.split(": ");
        line_split.next().unwrap();
        let nums = line_split.next().unwrap();
        let mut split = nums.split(" | ");
        let win: HashSet<&str> = split.next().unwrap().split(" ").filter(|&x| x != "" ).collect();
        let score = split.next().unwrap().split(" ").filter( |&x| x != "" && win.contains(x) ).collect::<Vec<&str>>();
        vec[idx] += 1;
        if score.len() > 0 {
            if idx == 0 {
                for i in 1..=score.len() {
                    vec[i] += 1;
                }
            } else {
                for i in idx+1..=idx+score.len() {
                    vec[i] += vec[idx];
                }
            }
        }
        idx += 1;
    }

    let sum: usize = vec.iter().sum();
    println!("{sum}");
}
