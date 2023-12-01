use std::fs::read_to_string;

fn main() {
    let mut sum: u64 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
    //for line in read_to_string("sample_input.txt").unwrap().lines() {
        let mut nums: Vec<u8> = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                nums.push(c as u8 - '0' as u8);
            }
        }
        sum += (10 * nums[0] + nums[nums.len() -1]) as u64;
    }
    println!("{sum}");
}
