use rand::Rng;
use std::collections::HashMap;

fn mean(ints: &Vec<i32>) -> f64 {
    let sum: i32 = ints.iter().sum();
    let len = ints.len();
    (sum as f64) / (len as f64)
}

fn median(ints: &Vec<i32>) -> i32 {
    let mid = ints.len() / 2;
    let mut sorted = ints.clone();
    sorted.sort_unstable();
    sorted[mid]
}

fn mode(ints: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for &i in ints {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_i = 0;
    let mut max_count = 0;
    for (i, count) in counts {
        if count > max_count {
            max_i = i;
            max_count = count;
        }
    }
    max_i
}

fn main() {
    let mut ints = vec![0; 100];
    for i in &mut ints {
        *i = rand::thread_rng().gen_range(0, 101);
    }
    println!("{:?}", ints);

    println!("Mean: {}, Median: {}, Mode: {}", mean(&ints), median(&ints), mode(&ints));
}
