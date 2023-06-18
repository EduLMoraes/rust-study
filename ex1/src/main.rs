/*
Exercício 1: Definindo uma Struct de Pessoa
Crie uma struct chamada Pessoa que represente
uma pessoa com os seguintes campos: nome (String),
idade (u32) e sexo (enum com valores "Masculino" e 
"Feminino"). Em seguida, escreva uma função imprimir_
dados que receba uma pessoa como parâmetro e imprima 
os seus dados na tela.
 */

#[derive(Debug)]
enum Sexo{Masculino = 0, Feminino = 1}

struct Pessoa{
    nome: String,
    idade: u32,
    sexo: Sexo,
}

impl Pessoa{
    fn new(nome: String, idade: u32, sexo: Sexo) -> Pessoa{
        Pessoa{ nome: nome,
                idade: idade,
                sexo: sexo}
    }

    fn print_data(&self){
        println!("Nome: {}\nidade: {}\nsexo: {:?}",
                    self.nome, self.idade, self.sexo)
    }
}

fn main() {
    let jhon = Pessoa::new("Jhon".to_string(), 42, Sexo::Masculino);
    let fern = Pessoa::new("Fernanda".to_string(), 27, Sexo::Feminino);
    jhon.print_data();
    fern.print_data();
}
