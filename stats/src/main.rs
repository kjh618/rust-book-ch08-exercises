use rand::Rng;

fn mean(ints: &Vec<i32>) -> f64 {
    let sum: i32 = ints.iter().sum();
    let len = ints.len();
    (sum as f64) / (len as f64)
}

fn median(ints: &Vec<i32>) -> i32 {
    0
}

fn mode(ints: &Vec<i32>) -> i32 {
    0
}

fn main() {
    let mut ints = vec![0; 100];
    for i in &mut ints {
        *i = rand::thread_rng().gen_range(0, 101);
    }
    println!("{:?}", ints);

    println!("Mean: {}, Median: {}, Mode: {}", mean(&ints), median(&ints), mode(&ints));
}
