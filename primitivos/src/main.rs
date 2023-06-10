use std::mem;

fn main() {
    // variavel Booleana
    let is_true: bool = true;
    let is_false = false;
    println!("bool: {is_true} {is_false}");

    // variavel Inteira
    let integer: i32 = -7;
    let integer2 = 7;
    let integer3 = 7i32;
    println!("integer: {integer} {integer2} {integer3}");

    // variavel Inteira sem sinal
    let integer4: u32 = 7;
    let integer5 = 7u32;
    println!("integer positive: {integer4} {integer5}");

    // variavel Flutuante
    let float: f32 = 1.7;
    let float2 = 1.7;
    let float3 = 1.7f32;
    println!("float: {float} {float2} {float3}");

    // variavel Caracter
    let string: String = "Hello my friend".to_string();
    let string2 = "Hello my friend".to_string();
    let string3: &str = "!";
    println!("Character: {string} {string2} {string3}");
    
    // variavel Tupla
    let tupla = (1, true, "Hi man", 1.7);
    println!("Tupla: {:?}", tupla);

    // variavel mutável
    let mut mutavel = false;
    let mut mutavel2:bool = true;
    let mut mutavel3:i32 = -1;
    let mut mutavel4 = 1;
    let mut mutavel5: String = "Hello world!".to_string();
    let mut mutavel6 = "Hello World!".to_string();
    let mut mutavel7: &str = "Hi";
    let mut mutavel8 = (-7, true, "hi", 1.8);
    mutavel = true;
    mutavel2 = false;
    mutavel3 = 45;
    mutavel4 = -78;
    mutavel5 = "Bye Bye".to_string();
    mutavel6 = "Goodbye World!".to_string();
    mutavel7 = "Bye";
    mutavel8 = (89, false, "bye", -45.990091);
    println!("mutavel: {mutavel}, {mutavel2}, {mutavel3}, {mutavel4}, {mutavel5}, {mutavel6}, {mutavel7}, {:?}", mutavel8);


    // adição inteira
    println!("1 + 2 = {}", 1u32 + 2);

    // subtração inteira
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // notação cientifica
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // pequeno circuito lógica
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // operações bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u16 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Usando anderlines para tornar mais levigel o número
    println!("One million is written as {}", 1_000_000u32);

    // Array
    let array = [1i32; 5];
    println!("Array: {:?}", array);
    println!("Array: {} items", array.len());
    println!("Array: {} bytes", mem::size_of_val(&array));

    
}
