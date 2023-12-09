use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {

    let mut sum = 0;

    //for line in read_to_string("sample_input.txt").unwrap().lines() {
    for line in read_to_string("input.txt").unwrap().lines() {
        let input: Vec<isize> = line.split(" ").map(|x| x.parse::<isize>().expect("input should be numbers")).collect();
        let mut diffs: Vec<Vec<isize>> = vec![input];

        while diffs.last().unwrap().iter().filter(|&x| *x == 0).collect::<Vec<_>>().len() != diffs.last().unwrap().len() {
            diffs.push(diffs.last().unwrap().windows(2).map( |x| x[1] - x[0]).collect());
        }

        let mut diff: isize = 0;
        for vec in diffs.iter().rev() {
            diff = vec[0] - diff;
        }
        sum += diff;
    }

    println!("Sum : {sum}");
}
