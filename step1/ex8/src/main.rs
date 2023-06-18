use std::io;

fn fibonacci(num: i32) -> i64 {
    if num < 2{
        return num.into();
    }
    return fibonacci(num-1)+fibonacci(num-2);
}


fn main() {

    println!("Informe um número:");

    let mut num = String::new();
    io::stdin().read_line(&mut num);
    let num: i32 = num.trim().parse().unwrap();

    let mut result: i64 = 0;
    result = fibonacci(num);
       
    println!("Resultado: {}", result);

}
