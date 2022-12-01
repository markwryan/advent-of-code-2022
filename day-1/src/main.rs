use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut cal_count = 0;
    let mut cal_max = 0;
    for i in lines {
        if i != "" {
            cal_count += i.parse::<i32>().unwrap();
        }
        else {
            if cal_count > cal_max{
                cal_max = cal_count;
            }
            cal_count = 0;
        }
    }
    println!("{}", cal_max);
}
