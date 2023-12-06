use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let time: usize;
    let dist: usize;
    let input = read_to_string("input.txt").unwrap();
    //let input = read_to_string("sample_input.txt").unwrap();
    let mut iter = input.lines();
    time = iter.next().expect("should have first line").trim_start_matches("Time: ").split(" ").filter(|x| *x != "").collect::<String>().parse::<usize>().expect("Distance should be a number");
    dist = iter.next().expect("should have second line").trim_start_matches("Distance: ").split(" ").filter(|x| *x != "").collect::<String>().parse::<usize>().expect("Distance should be a number");

    let mut num_wins = 0;
    for i in 1..time-1 {
        if i * (time-i) > dist {
            num_wins += 1;
        }
    }
    println!("Wins : {num_wins}");
}
