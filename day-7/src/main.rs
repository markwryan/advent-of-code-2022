use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = content.lines();
    // Track current position in file system
    let mut pwd: Vec<String> = vec![];
    // Flat map of all directories
    let mut filesystem: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let input: Vec<_> = line.split_whitespace().collect();
        // Changing into child directory
        if input[0] == "$" && input[1] == "cd" && input[2] != ".." {
            // new directory name is the previous + / + the file input
            let mut current_dir = String::new();
            if input[2] != "/" {
                current_dir = pwd.clone().last().expect("should exist").to_string();
            }
            let directory_name = current_dir + "/" + &input[2].to_string();
            // copy this value to update the present working dir
            let present_directory = directory_name.clone();
            // insert record into the map. Initial size of 0
            filesystem.insert(directory_name, 0);
            // push the new current working directory
            pwd.push(present_directory);
        }
        // Changing back out of directory
        else if input[0] == "$" && input[1] == "cd" && input[2] == ".." {
            // remove the most current pwd
            pwd.pop();
        } else if input[0] == "$" && input[1] == "ls" {
            // dont care
        } else if input[0] == "dir" {
            //dont care
        }
        // Line is a file output
        else {
            //parse filesize to usize
            let file_size: usize = input[0].parse().unwrap();
            // copy current pwd to update all
            let update_paths = pwd.clone();
            for v in update_paths {
                // duplicate and update the entry current size and insert the updated value
                let mut size = filesystem.get(&v).expect("Directory should exist.").clone();
                size = size + file_size;
                filesystem.insert(v, size);
            }
        }
    }
    // Find and output pt 1
    let mut total = 0;
    for (_k, v) in &filesystem {
        if v <= &100000 {
            total = total + v;
        }
    }
    println!("Part 1: {total}");

    // Find and output pt 2
    let mut del_directories: Vec<usize> = vec![];
    let root_total = filesystem.get("//").expect("Directory exists").clone();
    // Given total available space is 70000000, subtract root size to get whats free
    let free_space = 70000000 - root_total;

    for (_k, v) in filesystem {
        // 30000000 free needed for the update
        if free_space + v >= 30000000 {
            del_directories.push(v);
        }
    }
    // sort file sizes and output the smallest.
    del_directories.sort();
    println!("Part 2: {}", del_directories[0]);
}
