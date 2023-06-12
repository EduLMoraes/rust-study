use std::io;

fn main() {
    let mut num = String::new();

    println!("Informe um número para saber se é par ou ímpar");
    
    io::stdin().read_line(&mut num);

    let num:i32 = num.trim().parse().unwrap();
    
    if num%2 == 0{
        println!("Número é par");
    }
    else{
        println!("Número é ímpar");
    }
}
