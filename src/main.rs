use std::fs;

// pseudo code
// read file
// for each line
// for each char
// check 3 right, 3 left, 3 down, 3 up, 3 up-right, 3 up-left, 3 down-right, 3 down-left is X M A S
// replace all not X M A S with .
// print the result

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("should have been able to read the file");
}
