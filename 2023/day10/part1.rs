use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {

    //let input: Vec<Vec<char>> = read_to_string("sample_input.txt").unwrap().lines().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let input: Vec<Vec<char>> = read_to_string("input.txt").unwrap().lines().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut start: Option<(usize, usize)> = None;
    for (i, row) in input.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if *e == 'S' {
                start = Some((i,j));
            }
        }
    }

    let start = start.expect("Start should be found.");
    let height = input.len();
    let width = input[0].len();
    let up = vec!['7', '|', 'F'];
    let down = vec!['L', '|', 'J'];
    let left = vec!['L', '-', 'F'];
    let right = vec!['7', '-', 'J'];
    let mut visited: Vec<Vec<bool>> = vec![vec![false;width];height];
    // (x, y, dist, prev)
    let mut stack: Vec<(usize, usize, usize)> = vec![(start.0, start.1, 0)];
    let mut i = 0;
    let mut max = 0;
    while i < stack.len() {
        let current = stack[i];
        i += 1;

        if input[current.0][current.1] == '.' {
            continue;
        }

        visited[current.0][current.1] = true;
        max = std::cmp::max(max, current.2);

        match input[current.0][current.1] {
            '-' => {
                // Check left
                if current.1 != 0 && !visited[current.0][current.1 - 1] && left.contains(&input[current.0][current.1 - 1]) {
                    stack.push((current.0, current.1 - 1, current.2 + 1));
                }

                // Check right
                if current.1+1 < width && !visited[current.0][current.1 + 1] && right.contains(&input[current.0][current.1 + 1]) {
                    stack.push((current.0, current.1 + 1, current.2 + 1));
                }
            },
            '|' => {
                // Check up
                if current.0 != 0 && !visited[current.0 - 1][current.1] && up.contains(&input[current.0 - 1][current.1]) {
                    stack.push((current.0 - 1, current.1, current.2 + 1));
                }

                // Check down
                if current.0+1 < height && !visited[current.0 + 1][current.1] && down.contains(&input[current.0 + 1][current.1]) {
                    stack.push((current.0 + 1, current.1, current.2 + 1));
                }
            },
            'L' => {
                // Check up
                if current.0 != 0 && !visited[current.0 - 1][current.1] && up.contains(&input[current.0 - 1][current.1]) {
                    stack.push((current.0 - 1, current.1, current.2 + 1));
                }

                // Check right
                if current.1+1 < width && !visited[current.0][current.1 + 1] && right.contains(&input[current.0][current.1 + 1]) {
                    stack.push((current.0, current.1 + 1, current.2 + 1));
                }
            },
            'J' => {
                // Check up
                if current.0 != 0 && !visited[current.0 - 1][current.1] && up.contains(&input[current.0 - 1][current.1]) {
                    stack.push((current.0 - 1, current.1, current.2 + 1));
                }

                // Check left
                if current.1 != 0 && !visited[current.0][current.1 - 1] && left.contains(&input[current.0][current.1 - 1]) {
                    stack.push((current.0, current.1 - 1, current.2 + 1));
                }
            },
            '7' => {
                // Check down
                if current.0+1 < height && !visited[current.0 + 1][current.1] && down.contains(&input[current.0 + 1][current.1]) {
                    stack.push((current.0 + 1, current.1, current.2 + 1));
                }

                // Check left
                if current.1 != 0 && !visited[current.0][current.1 - 1] && left.contains(&input[current.0][current.1 - 1]) {
                    stack.push((current.0, current.1 - 1, current.2 + 1));
                }
            },
            'F' => {
                // Check down
                if current.0+1 < height && !visited[current.0 + 1][current.1] && down.contains(&input[current.0 + 1][current.1]) {
                    stack.push((current.0 + 1, current.1, current.2 + 1));
                }

                // Check right
                if current.1+1 < width && !visited[current.0][current.1 + 1] && right.contains(&input[current.0][current.1 + 1]) {
                    stack.push((current.0, current.1 + 1, current.2 + 1));
                }
            },
            'S' => {
                // Check down
                if current.0+1 < height && down.contains(&input[current.0 + 1][current.1]) {
                    stack.push((current.0 + 1, current.1, current.2 + 1));
                }

                // Check right
                if current.1+1 < width && right.contains(&input[current.0][current.1 + 1]) {
                    stack.push((current.0, current.1 + 1, current.2 + 1));
                }

                // Check up
                if current.0 != 0 && up.contains(&input[current.0 - 1][current.1]) {
                    stack.push((current.0 - 1, current.1, current.2 + 1));
                }

                // Check left
                if current.1 != 0 && left.contains(&input[current.0][current.1 - 1]) {
                    stack.push((current.0, current.1 - 1, current.2 + 1));
                }
            },
            _ => { unreachable!(); },
        }
    }

    println!("Max : {max}");
}
