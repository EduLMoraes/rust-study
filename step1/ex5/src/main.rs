use std::io;

fn main() {
    println!("Informe um tamanho para o vetor:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input :usize = input.trim().parse().unwrap();

    println!("{:?}",input);

    let mut vetor: Vec<i32> = Vec::new();

    println!("Informe {input} números:");


    for i in 0..input{
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();

        let num = num.trim().parse().unwrap();
        vetor.push(num);
    }

    let mut cont:i32 = 0;
    let mut var :i32 = i32::MAX;

    for i in 0..vetor.len(){
        for j in 0..vetor.len(){
            if vetor[i] == vetor[j]{
                cont += 1;
                var = vetor[j];
            }
        }
        if cont > 1{
            println!("{:?} apareceu {cont} vezes", vetor[i]);
        }
        cont = 0;
    }

}
