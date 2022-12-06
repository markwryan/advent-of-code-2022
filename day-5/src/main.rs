use std::fs;

fn main() {
    // Pass on parsing these from the input :)
    let mut p1_stacks = [
        vec!['_'],
        vec!['W', 'M', 'L', 'F'],
        vec!['B', 'Z', 'V', 'M', 'F'],
        vec!['H', 'V', 'R', 'S', 'L', 'Q'],
        vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'],
        vec!['L', 'S', 'W'],
        vec!['F', 'V', 'P', 'M', 'R', 'J', 'W'],
        vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F'],
        vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'],
        vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'],
    ];

    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    for line in content.lines() {
        let output = line
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");
        let line_move: Vec<&str> = output.split(",").collect();

        let stack_amt: usize = line_move[0].parse().unwrap();
        let stack_from: usize = line_move[1].parse().unwrap();
        let stack_to: usize = line_move[2].parse().unwrap();

        for _ in 0..stack_amt {
            if p1_stacks[stack_from].len() > 0 {
                let cr = p1_stacks[stack_from]
                    .pop()
                    .expect("stack should not be empty.");
                p1_stacks[stack_to].push(cr);
            }
        }
    }
    println!("Part 1 Stacks:");
    for s in p1_stacks {
        for x in s {
            print!("{}", x);
        }
        println!("");
    }

    // Pass on parsing these from the input :)
    let mut p2_stacks = [
        vec!['_'],
        vec!['W', 'M', 'L', 'F'],
        vec!['B', 'Z', 'V', 'M', 'F'],
        vec!['H', 'V', 'R', 'S', 'L', 'Q'],
        vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'],
        vec!['L', 'S', 'W'],
        vec!['F', 'V', 'P', 'M', 'R', 'J', 'W'],
        vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F'],
        vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'],
        vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'],
    ];
    for line in content.lines() {
        let output = line
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");
        let line_move: Vec<&str> = output.split(",").collect();

        let stack_amt: usize = line_move[0].parse().unwrap();
        let stack_from: usize = line_move[1].parse().unwrap();
        let stack_to: usize = line_move[2].parse().unwrap();

        let mut cr: Vec<char> =
            p2_stacks[stack_from].split_off(p2_stacks[stack_from].len() - stack_amt);
        p2_stacks[stack_to].append(&mut cr);
    }
    println!("Part 2 Stacks:");
    for s in p2_stacks {
        for x in s {
            print!("{}", x);
        }
        println!("");
    }
}
