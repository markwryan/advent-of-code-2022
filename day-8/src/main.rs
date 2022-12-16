use std::fs;

fn main() {
    const WIDTH: usize = 99;
    const HEIGHT: usize = 99;
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut tree_map: [[u32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut row = 0;
    let mut scenic_score: Vec<i32> = vec![];

    for line in content.lines() {
        let mut column = 0;
        for tree in line.chars() {
            tree_map[row][column] = tree.to_digit(10).unwrap();
            column += 1;
        }
        row += 1;
    }

    let mut visible = 0;
    for r in 1..HEIGHT - 1 {
        for c in 1..WIDTH - 1 {
            let t = tree_map[r][c];
            let mut north = true;
            let mut north_count = 0;
            let mut n: i32 = r as i32 - 1;
            while n >= 0 {
                north_count += 1;
                if t <= tree_map[n as usize][c] {
                    north = false;
                    break;
                }
                n -= 1;
            }

            let mut south = true;
            let mut south_count = 0;
            for s in r + 1..HEIGHT {
                south_count += 1;
                if t <= tree_map[s][c] {
                    south = false;
                    break;
                }
            }

            let mut west = true;
            let mut west_count = 0;
            let mut w = c as i32 - 1;
            while w >= 0 {
                west_count += 1;
                if t <= tree_map[r][w as usize] {
                    west = false;
                    break;
                }
                w -= 1;
            }

            let mut east = true;
            let mut east_count = 0;
            for e in c + 1..WIDTH {
                east_count += 1;
                if t <= tree_map[r][e] {
                    east = false;
                    break;
                }
            }

            if north || south || east || west {
                visible += 1;
            }
            let scenic = north_count * south_count * east_count * west_count;
            scenic_score.push(scenic);
        }
    }
    visible += 99 + 99 + 97 + 97;
    println!("Part 1 {visible}");
    scenic_score.sort();
    println!("Part 2 {}", scenic_score.last().expect("vector not empty"));
}
