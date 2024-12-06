
fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("should have been able to read the file");
    let dos: Vec<&str> =  contents.split("do()").collect();
    let redo: Vec<&str> =  contents.split("redo()").collect();
    let muls: Vec<&str> =  contents.split("mul(").collect();
    let mut sum = 0;
    let mut redo = false;
    for mul in muls {
        if redo {
            continue;
        }
        let separator: Vec<&str> = mul.split(|c| c == ')' || c == ',').collect();
        if let Some(x) = separator[0].parse::<i32>().ok() {
            if let Some(y) = separator[1].parse::<i32>().ok() {
                sum += x * y;
            }
        }
    }
    println!("{}", sum);
}
