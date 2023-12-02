use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    //for line in read_to_string("sample_input.txt").unwrap().lines() {
    for line in read_to_string("input.txt").unwrap().lines() {
        // r g b
        let mut max = [0, 0, 0];
        let mut split = line.split(": ");
        let _id = &split.next().unwrap()[5..].parse::<usize>().unwrap();
        for draw in split.next().unwrap().split("; ") {
            for count in draw.split(", ") {
                let mut count_split = count.split(" ");
                //println!("{count_split:?}");
                let n = count_split.next().unwrap().parse::<usize>().unwrap();
                match count_split.next().unwrap() {
                    "blue" => {
                        max[2] = std::cmp::max(max[2], n);
                    },
                    "red" => {
                        max[0] = std::cmp::max(max[0], n);
                    },
                    "green" => {
                        max[1] = std::cmp::max(max[1], n);
                    },
                    _ => {
                        println!("error");
                    }
                }
            }
        }
        //println!("{max:?}");
        sum += max[0] * max[1] * max[2];
    }
    println!("sums = {sum}");
}
