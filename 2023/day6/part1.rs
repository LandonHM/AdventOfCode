use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let mut sum: usize = 1;
    let times: Vec<usize>;
    let dists: Vec<usize>;
    let input = read_to_string("input.txt").unwrap();
    //let input = read_to_string("sample_input.txt").unwrap();
    let mut iter = input.lines();
    times = iter.next().expect("should have first line").trim_start_matches("Time: ").split(" ").filter(|x| *x != "").map(|x| x.parse::<usize>().expect("Times should all be numbers")).collect();
    dists = iter.next().expect("should have second line").trim_start_matches("Distance: ").split(" ").filter(|x| *x != "").map(|x| x.parse::<usize>().expect("Distances should all be numbers")).collect();
    for (time, dist) in times.iter().zip(dists.iter()) {
        let mut num_wins = 0;
        for i in 1..time-1 {
            if i * (time-i) > *dist {
                num_wins += 1;
            }
        }
        if num_wins > 0 {
            sum *= num_wins;
        }
    }
    println!("Sum : {sum}");
}
