use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input: Vec<char> = content.chars().collect();

    let mut output = 0;

    for i in 3..input.len() {
        let one = input[i - 3];
        let two = input[i - 2];
        let three = input[i - 1];
        let four = input[i];

        if one != two && one != three && one != four && two != three && two != four && three != four
        {
            output = i + 1;
            break;
        }
    }

    let mut pt2_output = 0;
    for i in 13..input.len() {
        let mut t: Vec<char> = input[i - 13..i + 1].to_vec();

        while t.len() > 0 {
            let x = t.pop().unwrap();
            if t.contains(&x) {
                break;
            }
        }

        if t.len() == 0 {
            pt2_output = i + 1;
            break;
        }
    }

    println!("Part 1: {}", output);
    println!("Part 2: {}", pt2_output);
}
