use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("not found");
    let data = data.split("\n");
    for line in data {
        if line.len() == 0 {
            continue
        }
        let arr = line.split(",");
        let mut sum = 0;
        for a in arr {
            let num = a.parse::<i32>().unwrap();
            sum = num + sum;
        }
        println!("sum is: {}",sum);
    }
}
