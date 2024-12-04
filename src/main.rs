use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("should have been able to read the file");
    let mut count = 0;
    for line in contents.lines() {
        let mut vec: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("parse failed")).collect();
        let mut idx = 1;
        let mut dec = true;
        let mut tolerate = true;
        let mut inc = true;
        let mut safe = true;
        while idx < vec.len() - 1 {
           if vec[idx - 1] < vec[idx] || vec[idx - 1] - vec[idx] > 3 {
               if tolerate && vec[idx - 1] < vec[idx + 1] && vec[idx + 1] - vec[idx - 1] < 3 {
                   vec.remove(idx);
                    tolerate = false;
                    continue;
               }
               dec = false;
                break;
           }
           idx += 1;
        }
        idx = 1;
        while idx < vec.len() - 1 {
           if vec[idx - 1] > vec[idx] || vec[idx] - vec[idx - 1] > 3 {
               if tolerate && vec[idx - 1] > vec[idx + 1] && vec[idx - 1] - vec[idx + 1] < 3 {
                   vec.remove(idx);
                    tolerate = false;
                    continue;
               }
               inc = false;
                break;
           }
           idx += 1;
        }
        idx = 1;
        while idx < vec.len() - 1 {
           if vec[idx - 1] == vec[idx] {
               if tolerate && vec[idx - 1] != vec[idx + 1] {
                   vec.remove(idx);
                    tolerate = false;
                    continue;
               }
                safe = false;
                break;
           }
           idx += 1;
        }
        if (inc || dec) && safe {
            count += 1;
            println!("Safe: {:?}", vec);
        } else {
            println!("Unsafe: {:?}", vec);
        }
    }
    println!("{}", count);
}
