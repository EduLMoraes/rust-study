use std::io;

pub fn profit() -> f64{
    let mut dividend = String::new();
    let mut time = String::new();

    println!("Qual é o valor do dividendo?");
    io::stdin().read_line(&mut dividend).unwrap();

    println!("Qual a estimativa de tempo?");
    io::stdin().read_line(&mut time).unwrap();

    let dividend: f64 = dividend.trim().parse().unwrap();
    let time: f64 = time.trim().parse().unwrap();

    return dividend * time;
}
