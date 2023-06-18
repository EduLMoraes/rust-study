/*
Exercício 3: Struct para Retângulo
Crie uma struct chamada Retangulo
que represente um retângulo com os
campos largura (f32) e altura (f32). 
Em seguida, implemente os seguintes
métodos para a struct:

    area: retorna a área do retângulo (largura * altura).
    perimetro: retorna o perímetro do retângulo (2 * largura + 2 * altura).
    eh_quadrado: retorna um valor booleano indicando se o retângulo é um quadrado (largura == altura).
*/

struct Retangulo{
    altura: f32,
    largura: f32,
}

impl Retangulo{
    fn new(altura: f32, largura: f32) -> Retangulo{
        Retangulo { altura: altura, largura: largura }
    }

    fn area(&self){
        println!("Area: {}", self.altura*self.largura);  
    }

    fn perimetro(&self){
        println!("Perimetro: {}", 2.0 * self.largura + 2.0 * self.altura);
    }

    fn is_quadrado(&self){
        println!("É quadrado? {}", self.altura == self.largura);
    }
}

fn main() {
    let retangulo = Retangulo::new(31.0, 32.0);
    retangulo.area();
    retangulo.perimetro();
    retangulo.is_quadrado();
}
