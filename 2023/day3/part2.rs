use std::fs::read_to_string;

fn main() {
    let mut schem: Vec<Vec<char>> = vec![];
    let mut total = 0;
    //for line in read_to_string("sample_input.txt").unwrap().lines() {
    for line in read_to_string("input.txt").unwrap().lines() {
        schem.push(line.chars().collect());
    }
    for i in 0..schem.len() {
        for j in 0..schem[i].len() {
            if schem[i][j] == '*' {
                schem[i][j] = '.';

                let mut cnt = 0;
                let mut sum = 1;
                let mut seen: Vec<Vec<bool>> = vec![];
                for i in 0..schem.len() {
                    seen.push(vec![false;schem[i].len()]);
                }
                sum *= count(&mut seen, &schem, i-1, j-1, &mut cnt);
                sum *= count(&mut seen, &schem, i-1, j, &mut cnt);
                sum *= count(&mut seen, &schem, i-1, j+1, &mut cnt);

                sum *= count(&mut seen, &schem, i+1, j-1, &mut cnt);
                sum *= count(&mut seen, &schem, i+1, j, &mut cnt);
                sum *= count(&mut seen, &schem, i+1, j+1, &mut cnt);

                sum *= count(&mut seen, &schem, i, j+1, &mut cnt);
                sum *= count(&mut seen, &schem, i, j-1, &mut cnt);
                if cnt == 2 {
                    total += sum;
                }

            }
        }
    }
    println!("{total}");
}

fn count(seen: &mut Vec<Vec<bool>>, mat: &Vec<Vec<char>>, i: usize, j: usize, cnt: &mut usize) -> usize {
    if let Some(row) = mat.get(i) {
        if let Some(e) = row.get(j) {
            if e.is_numeric()  && !seen[i][j] {
                // Get to the right end of the number
                *cnt += 1;
                let mut j = j;
                while j < row.len() && row[j].is_numeric() { j += 1; }
                j -= 1;
                let mut num: Vec<char> = vec![];
                while j > 0 && row[j].is_numeric() {seen[i][j] = true; num.push(row[j]); j -= 1; }
                if j == 0 && row[0].is_numeric() {seen[i][j] = true;num.push(row[0]); }
                let mut exp = 1;
                let mut ret = 0;
                for n in num {
                    ret += exp * (n as usize - '0' as usize);
                    exp *= 10;

                }
                return ret;
            }
        }
    }
    1
}
