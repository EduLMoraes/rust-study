use std::io;

pub fn value_finish() -> f64{
    let mut value = String::new();
    let mut dividend = String::new();
    let mut time = String::new();

    println!("Qual era o valor inicial?");
    io::stdin().read_line(&mut value).unwrap();
    println!("Qual é o valor do dividendo?");
    io::stdin().read_line(&mut dividend).unwrap();
    println!("Qual o tempo a ser estimado?");
    io::stdin().read_line(&mut time).unwrap();

    let value: f64= value.trim().parse().unwrap();
    let dividend: f64 = dividend.trim().parse().unwrap();
    let time: f64 = time.trim().parse().unwrap();

    return (dividend * time) + value;
}
