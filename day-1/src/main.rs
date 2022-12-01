use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let lines = content.lines();
    let mut cal_count = 0;
    let mut elves = Vec::<i32>::new();
    for i in lines {
        if i != "" {
            cal_count += i.parse::<i32>().unwrap();
        }
        else {
            elves.push(cal_count);
            cal_count = 0;
        }
    }
    elves.sort();
    elves.reverse();
    // Part 1
    println!("Part 1: {}", elves[0]);
    // Part 2
    println!("Part 2: {}", elves[0]+elves[1]+elves[2]);
}
