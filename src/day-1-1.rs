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

    let mut idx = 0;
    let mut result = vec![];
    let mut sum = 0;
    left.sort();
    right.sort();
    while idx < left.len() {
       let r: i32 = right[idx] - left[idx];
       result.push(r.abs());
       idx += 1;
    }
    for num in result {
        sum += num;
    }

    println!("{}", sum);
}
