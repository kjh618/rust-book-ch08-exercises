use std::io;

fn is_vowel(letter: &str) -> bool {
    let vowels = ["a", "e", "i", "o", "u"];
    vowels.contains(&letter)
}

fn main() {
    println!("Enter words: ");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence)
        .expect("Failed to read line");
    
    let words: Vec<_> = sentence.trim().split_whitespace().collect();
    for word in words {
        if is_vowel(&word[0..1]) {
            print!("{}hay ", word);
        } else {
            print!("{}{}ay ", &word[1..], &word[0..1]);
        }
    }
    println!();
}
