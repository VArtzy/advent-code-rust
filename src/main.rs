use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("should have been able to read the file");
    let muls: Vec<&str> =  contents.split("mul(").collect();
    let mut sum = 0;
    for mul in muls {
        let separator: Vec<&str> = mul.split(|c| c == ')' || c == ',').collect();
        if let Some(x) = separator[0].parse::<i32>().ok() {
            if let Some(y) = separator[1].parse::<i32>().ok() {
                sum += x * y;
            }
        }
    }
    println!("{}", sum);
}
