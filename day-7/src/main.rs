use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = content.lines();
    let mut pwd: Vec<String> = vec!();
    let mut filesystem: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let input: Vec<_> = line.split_whitespace().collect();
        // Changing into child directory
        if input[0] == "$" && input[1] == "cd" && input[2] != ".." {
            if input[2] != "/" {
                let directory_name = pwd.clone().join("") + "/" + &input[2].to_string();
                let present_directory = directory_name.clone();
                filesystem.insert(directory_name, 0);
                pwd.push(present_directory);
            }
            else {
            }
                   
        }
        else if input[0] == "$" && input[1] == "cd" && input[2] == ".." {
            pwd.pop();
        }
        else if input[0] == "$" && input[1] == "ls" {
            // dont care
        }
        else if input[0] == "dir" {
            //dont care
        }
        else {
            let file_size: usize = input[0].parse().unwrap();
            let update_paths = pwd.clone();
            for v in update_paths {
                let mut size = filesystem.get(&v).expect("Directory should exist.").clone();
                size = size + file_size;
                filesystem.insert(v, size);
            }
        }
    }
    let mut total = 0;
    for (k,v) in filesystem {
        if v <= 100000 {
            total = total + v;
        }
    }
    println!("Part 1: {total}");


}