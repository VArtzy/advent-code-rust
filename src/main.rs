fn main() {
    let mut left = vec![3, 4, 2, 1, 3, 3];
    let mut right = vec![4, 3, 5, 3, 9, 3];
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
