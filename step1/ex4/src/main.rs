use std::io;

fn main() {
    let mut input = String::new();

    println!("Informe um número para saber se é primo:");
    io::stdin().read_line(&mut input).unwrap();

    let input : i32 = input.trim().parse().unwrap(); 

    let mut cont: i8 = 0;

    for i in 1..=input{
        if i*2 == input || i == input{
            println!("{i}");
            cont += 1;
        }
    }

    println!("Existem {cont} divisores para esse número além de 1");

    if cont > 1{
        println!("O número {input} não é primo!");
    }
    else{
        println!("O número {input} é primo!");
    }

}
