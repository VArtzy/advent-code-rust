fn main() {
    let mut left = vec![3, 4, 2, 1, 3, 3];
    let mut right = vec![4, 3, 5, 3, 9, 3];
    left.sort();
    right.sort();
    for num in left.iter() {
        println!("{:?}", num + right.iter().next().unwrap_or_default());
    }
}
