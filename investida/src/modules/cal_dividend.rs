use std::io;

pub fn dividend() -> f64{
    let mut value = String::new();
    let mut div_yield = String::new();
    
    println!("Qual a quantia inicial?");
    io::stdin().read_line(&mut value).unwrap();
    println!("Qual a % de retorno?");
    io::stdin().read_line(&mut div_yield).unwrap();

    let value: f64 = value.trim().parse().unwrap();
    let div_yield: f64 = div_yield.trim().parse().unwrap();

    return ((div_yield/100.0) * value)/10.0;
}

