fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("should have been able to read the file");
    let split: Vec<i32> = contents.split_ascii_whitespace().map(|s| s.parse().expect("failed parse")).collect();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut counter = 0;
    for num in split {
        counter += 1;
        if counter % 2 == 0 {
            right.push(num); 
        } else {
            left.push(num);
        }
    }

    let mut result = 0;
    for num in left {
        result += num as usize * right.iter().filter(|&&x| x == num).count();
    }
    println!("{}", result);
}
