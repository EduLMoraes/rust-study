use std::io;

fn main() {
    let mut palavra: String = String::new();

    println!("Informe uma palavra para saber se é políndromo:");
    io::stdin().read_line(&mut palavra).unwrap();

    let palavra: &str = palavra.trim();

    let inversa: String = palavra.chars().rev().collect::<String>();

    if  palavra == inversa{
        println!("Esta palavra é um políndromo");
    }
    else{
        println!("Esta palavra não é um políndromo");
    }

}
