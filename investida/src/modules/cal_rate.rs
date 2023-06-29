use std::io;

pub fn rate() -> f64{
    let mut value = String::new();
    let mut dividend = String::new();

    println!("Informe o valor investido");
    io::stdin().read_line(&mut value).unwrap();

    println!("Informe o dividendo recebido");
    io::stdin().read_line(&mut dividend).unwrap();

    let value: f64 = value.trim().parse().unwrap();
    let dividend: f64 = dividend.trim().parse().unwrap();

    return ((dividend*100.0) / value) - 1.0;
}
