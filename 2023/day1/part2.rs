use std::fs::read_to_string;

fn main() {
    //let nums_str = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u64 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
    //for line in read_to_string("sample_input_2.txt").unwrap().lines() {
        let mut nums: Vec<u8> = vec![];
        let mut i: usize = 0;
        while i < line.len() {
            let c = line.get(i..i+1).unwrap();
            let c = c.chars().next().unwrap();
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    nums.push(c as u8 - '0' as u8);
                },
                'o' => {
                    if (i + 3) <= line.len() && &line[i..i+3] == "one" {
                        nums.push(1 as u8);
                    }
                },
                't' => {
                    if (i + 3) <= line.len() && &line[i..i+3] == "two" {
                        nums.push(2 as u8);
                    } else if (i + 5) <= line.len() && &line[i..i+5] == "three" {
                        nums.push(3 as u8);
                    }
                },
                'f' => {
                    if (i + 4) <= line.len() && &line[i..i+4] == "four" {
                        nums.push(4 as u8);
                    } else if (i + 4) <= line.len() && &line[i..i+4] == "five" {
                        nums.push(5 as u8);
                    }
                },
                's' => {
                    if (i + 3) <= line.len() && &line[i..i+3] == "six" {
                        nums.push(6 as u8);
                    } else if (i + 5) <= line.len() && &line[i..i+5] == "seven" {
                        nums.push(7 as u8);
                    }
                },
                'e' => {
                    if (i + 5) <= line.len() && &line[i..i+5] == "eight" {
                        nums.push(8 as u8);
                    }
                },
                'n' => {
                    if (i + 4) <= line.len() && &line[i..i+4] == "nine" {
                        nums.push(9 as u8);
                    }
                },
                _ => { }
            }
            i += 1;
        }
        //println!("{nums:?}");
        sum += (10 * nums[0] + nums[nums.len() -1]) as u64;
    }
    println!("{sum}");
}
