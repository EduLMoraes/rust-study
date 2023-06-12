use std::io;
//use std::slice::range;
fn fatorial(num:i32) -> i32{
    let mut result:i32 = 2;

    for i in 2..num{
        result = result*(i+1);
    }

    return result;
}

fn main() {
    let mut num = String::new();
    println!("Insira um número para saber o fatorial dele");
    io::stdin().read_line(&mut num);

    let num:i32 = num.trim().parse().unwrap();

    let result = fatorial(num);

    println!("O fatorial de {}! é = {:?}", num, result);
}
