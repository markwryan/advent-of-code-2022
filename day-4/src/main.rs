use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut part1_total = 0;
    let mut part2_total = 0;
    for line in content.lines() {
        let p: Vec<&str> = line.split(',').collect();
        let p1: Vec<&str> = p[0].split("-").collect();
        let p2: Vec<&str> = p[1].split("-").collect();
        let one_lower: i32 = p1[0].parse().unwrap();
        let one_upper: i32 = p1[1].parse().unwrap();
        let two_lower: i32 = p2[0].parse().unwrap();
        let two_upper: i32 = p2[1].parse().unwrap();

        // Part 1: check contains
        if one_lower <= two_lower && one_upper >= two_upper {
            part1_total = part1_total + 1;
        }
        else if two_lower <= one_lower && two_upper >= one_upper {
            part1_total = part1_total + 1;
        }

        // Part 2: check overlap
        if one_lower <= two_lower && two_lower <= one_upper {
            part2_total = part2_total + 1;
        }
        else if two_lower <= one_lower && one_lower <= two_upper {
            part2_total = part2_total + 1;
        }

        
    }
    println!("Part 1 Total: {}", part1_total);
    println!("Part 2 Total: {}", part2_total);
}
