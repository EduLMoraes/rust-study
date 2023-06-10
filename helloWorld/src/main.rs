use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}: {}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    // Criando comentário comum
    /* Criando um bloco
        de comentário */



    println!("Hello, world!");
    println!("I am Eduardo");


    // Podemos alterar o formato do que será escrito com {:formato} por exemplo
    // {:b} para binário

    println!("Base octal {:o}", 17);
    println!("Base decimal {}", 17);
    println!("Base binária {:b}", 17);
    println!("Base binária {:b}", 17);
    println!("Base hexadecimal {:x}", 17);

    format!("Formato: hello world");

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    let foo: i64 = 3735928559;
    format!("{}", foo);
}
