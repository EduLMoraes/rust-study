mod modules{
    pub mod cal_dividend;
    pub mod cal_profit;
    pub mod cal_end;
    pub mod cal_rate;
}

use std::io;
use modules::{cal_dividend, cal_profit, cal_end, cal_rate};
use cal_dividend::dividend;
use cal_profit::profit;
use cal_end::value_finish;
use cal_rate::rate;


fn main(){
    loop{
        print_menu();

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim() == "5"{
            return;
        }

        if choice.trim() == "1"{
            println!("Seu dividendo é de {:.2?} por mês", dividend());
            println!("Press any key to continue...");

            let mut ok = String::new();
            io::stdin().read_line(&mut ok).unwrap();
        }

        if choice.trim() == "2"{
            println!("Sua taxa de lucro de dividendo é {:.2?}", rate());
            println!("Press any key to continue...");

            let mut ok = String::new();
            io::stdin().read_line(&mut ok).unwrap();
        }

        if choice.trim() == "3"{
            println!("Seu lucro é de {:.2?}", profit());
            println!("Press any key to continue...");

            let mut ok = String::new();
            io::stdin().read_line(&mut ok).unwrap();
        }

        if choice.trim() == "4"{
            println!("O valor ao final do periodo do investimento é {:.2?}", value_finish());
            println!("Press any key to continue...");

            let mut ok = String::new();
            io::stdin().read_line(&mut ok).unwrap();
        }
    }        
}

fn print_menu(){
    println!("|=======INVESTIDA=======|");
    println!("\nO que deseja?\n");
    println!("1) Calcular dividendo");
    println!("2) Calcular taxa");
    println!("3) Calcular lucro");
    println!("4) Calcular fundo final");
    println!("5) Sair");
}
