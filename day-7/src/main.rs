use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Directory {
    files: Vec<File>,
    parent: String,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i32,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = content.lines();
    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut current_directory_name = "/"; 
    for line in lines {
        let input: Vec<_> = line.split_whitespace().collect();
        let current_directory = directories.get(current_directory_name);
        if input[0] == "$" && input[1] == "cd" && input[2] != ".." {
            let dir = Directory {
                files: vec!(),
                parent: (&current_directory_name).to_string(),
            };
            directories.insert(input[2].to_string(), dir);
            current_directory_name = input[2];
        }
        else if input[0] == "$" && input[1] == "cd" && input[2] == ".." {
            current_directory_name = &current_directory.unwrap().parent;
        }
        else if input[0] == "$" && input[1] == "ls" {
            // dont care
        }
        else if input[0] == "dir" {
            //dont care
        }
        else {
            let f = File{name: input[1].to_string(), size: input[0].parse().unwrap()};
            let dir = current_directory.unwrap();

            let mut up = Directory {
                files: dir.files.to_vec(),
                parent: dir.parent.to_string(),
            };
            up.files.push(f);
            directories.insert(current_directory_name.to_string(), up);
        }
    }

    for (k, _v) in &directories {
        let s = calculate_size(&k, &directories);
        if s <= 100000 {
            println!("{}", k.to_string());
        }
    }
}

fn calculate_size(directory_name: &String , directories: &HashMap<String, Directory>) -> i32 {
    let mut total = 0;
    let d = directories.get(directory_name).expect("directory should exist");
    for file in d.files.iter() {
        total = total + file.size;
    }
    if d.parent != "_" {
        total = total + calculate_size(&d.parent, &directories);
    }
    
    return total;
}

