use std::io;

fn main() {
    println!("Informe uma palavra:");

    let mut first_word = String::new();
    io::stdin().read_line(&mut first_word).unwrap();

    let first_word: &str = first_word.trim();

    println!("Informe outra agora:");

    let mut second_word = String::new();
    io::stdin().read_line(&mut second_word).unwrap();

    let second_word: &str = second_word.trim();

    let mut letters_fw: Vec<String> = Vec::new();
    let mut letters_sw: Vec<String> = Vec::new();

    for i in first_word.chars(){
        letters_fw.push(i.to_string());
    }
    for i in second_word.chars(){
        letters_sw.push(i.to_string());
    }
    if letters_sw.len() != letters_fw.len() || first_word == second_word{
        println!("Aff, nada de interessante!");
    }
    else{
        let mut cont: i8 = 0;
        for i in 0..letters_fw.len(){
            for j in 0..letters_fw.len(){
                if letters_fw[i] == letters_sw[j]{
                    cont += 1;
                }
            }
        }
        if cont == letters_fw.len().try_into().unwrap(){
            println!("Olha um anagrama!!!");
        }
    }

}
