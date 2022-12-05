use std::fs;

fn main() {
    let key = ['_','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut part1_total = 0;
    for line in content.lines() {
        let compartments = line.split_at(line.len()/2);
        let mut compartment1: Vec<char> = compartments.0.chars().collect();
        let mut compartment2: Vec<char> = compartments.1.chars().collect();
        compartment1.sort();
        compartment2.sort();
        compartment1.dedup();
        compartment2.dedup();
        for piece in compartment1 {
            if compartment2.contains(&piece) {
                part1_total += key.iter().position(|&x| x == piece).unwrap();
            }
        }
    }
    println!("Part 1 Total: {}", part1_total);
}

