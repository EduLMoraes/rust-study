struct S1(i32, String, bool, u32);

#[derive(Debug)]
struct S2{
    atribute: String,
    atribute2: i32,
    atribute3: u32,
    atribute4: bool,
}

struct S3;

static statica: &str = "olha";
const constante:i32 = i32::MAX;

fn main() {
    // Existem 3 estruturas (struct) que podem serem feitas
    /*  
        S1) Tupla estruturada
        S2) Estruturas C clássicas
        S3) Unit Structs, Estruturas sem campos
     */

    let s2 = S2{atribute: "Hello world!".to_string(), atribute2: -45, atribute3: 40, atribute4: true};
    println!("{:?}", s2);
    println!("{}", s2.atribute);

    println!("statica: {statica} \n constante: {constante}")
}
