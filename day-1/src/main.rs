use std::fs;

fn main() {
    // Read in puzzle input from text file
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    //Split into a vector by line
    let lines = content.lines();
    // Tracks the total calories between line breaks (the total calories for one elf)
    let mut cal_count = 0;
    // Track each elf's total calories in a new vector of integers
    let mut elves = Vec::<i32>::new();
    // Iterate over all the input lines
    for i in lines {
        //Blank lines signify a new elf. Checking that it is the same elf still
        if i != "" {
            //Convert line to int and add to total for that elf
            cal_count += i.parse::<i32>().unwrap();
        } else {
            //We are at a new elf, so push that previous elf's total to the vector of elves and reset calorie total
            elves.push(cal_count);
            cal_count = 0;
        }
    }
    //Sort elves from least to most
    elves.sort();
    //Reverse to sort from most to least
    elves.reverse();
    // Part 1 -- elf with the most calories
    println!("Part 1: {}", elves[0]);
    // Part 2 -- sum of the top 3 elves calories
    println!("Part 2: {}", elves[0] + elves[1] + elves[2]);
}
