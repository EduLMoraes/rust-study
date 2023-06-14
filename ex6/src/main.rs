use std::io;

fn media(vetor: &Vec<i32>) -> i32{
    let mut soma = 0;

    for i in 0..vetor.len(){
        soma += vetor[i]
    }
    return soma/8;
}

fn distancia(media:i32, vetor: &Vec<i32>) -> i32{
    let mut dist = 0;
    let mut distances : Vec<i32> = Vec::new();
    let mut soma = 0;
    let resultado: i32;

    for i in 0..vetor.len(){
        dist = (vetor[i] - media)*(vetor[i] - media);
        distances.push(dist);
    }

    println!("{:?}", distances);

    for i in 0..distances.len(){
        soma += distances[i];
    }

    println!("{soma}");

    resultado = soma/8;

    return resultado;
}

fn main() {
    println!("Informe 8 numeros inteiros:");

    let mut vetor: Vec<i32> = Vec::new();
    
    for i in 0..8{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse().unwrap();
        vetor.push(input);
    }

    let media : i32 = media(&vetor);

    let desvio = distancia(media, &vetor);

    println!("A média é {media} com um desvio padrão de {desvio}");
}
