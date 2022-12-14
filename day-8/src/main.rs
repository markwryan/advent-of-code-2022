use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut tree_map:[[u32;100];99] = [[0;100];99];
    let mut row = 0;
    for line in content.lines() {
        let mut column = 0;
        for tree in line.chars() {
            tree_map[row][column] = tree.to_digit(10).unwrap();
            column += 1;
        }
        row += 1;
    }

    for r in 1..98 {
        for c in 1..99 {
            let t = tree_map[r][c];
            print!("{t}");
        }
        println!("");
    }
}
