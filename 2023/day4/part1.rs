use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let mut sum = 0;
    let two: i32 = 2;
    for line in read_to_string("input.txt").unwrap().lines() {
    //for line in read_to_string("sample_input.txt").unwrap().lines() {
        let mut line_split = line.split(": ");
        line_split.next().unwrap();
        let nums = line_split.next().unwrap();
        let mut split = nums.split(" | ");
        let win: HashSet<&str> = split.next().unwrap().split(" ").filter(|&x| x != "" ).collect();
        let score = split.next().unwrap().split(" ").filter( |&x| x != "" && win.contains(x) ).collect::<Vec<&str>>();
        if score.len() > 0 {
            sum += two.pow((score.len()-1) as u32) as usize;
        }
    }
    println!("{sum}");
}
