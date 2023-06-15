use std::io;
fn main() {
    println!("Informe uma palavra:");

    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();

    println!("{}", word.chars().rev().collect::<String>());
}
