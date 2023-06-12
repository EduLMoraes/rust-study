use std::io;
fn inverter(palavra:String) -> String{
    let mut inversa:String = String::new();
    
    for i in palavra.len()..=0{
        inversa.push(palavra.chars().nth((i as i32).try_into().unwrap()).unwrap());
    }

    return inversa;
}

fn main() {
    let mut palavra = String::new();
    let mut inversa:String;

    println!("Informe uma palavra para saber se é políndromo:");
    io::stdin().read_line(&mut palavra).unwrap();

    inversa = inverter(palavra.clone().to_string());

    println!("{}{}", palavra, inversa);

    if (inversa == palavra){
        println!("Esta palavra é um políndromo");
    }
    else{
        println!("Esta palavra não é um políndromo");
    }
}
