/*
Exercício 2: Enum para Status de Pedido
Crie um enum chamado StatusPedido com os 
seguintes valores possíveis: "AguardandoPagamento", 
"EmProcessamento", "Enviado" e "Entregue". Em seguida, 
escreva uma função atualizar_status que receba o status
atual de um pedido como parâmetro e retorne o próximo
status de acordo com a seguinte lógica:

    Se o status atual for "AguardandoPagamento", retorne "EmProcessamento".
    Se o status atual for "EmProcessamento", retorne "Enviado".
    Se o status atual for "Enviado", retorne "Entregue".
    Se o status atual for "Entregue", retorne "Entregue" (não há próximo status).
 */
#[derive(Debug)]
#[derive(PartialEq)]
enum StatusPedido{AguardandoPagamento = 0, EmProcessamento = 1, Enviado = 2, Entregue = 3}

fn main() {
    let mut status = StatusPedido::AguardandoPagamento;
    println!("Status atual: {:?}", status);
    status = atualizar_status(status);
    println!("Status atual: {:?}", status);
    status = atualizar_status(status);
    println!("Status atual: {:?}", status);
    status = atualizar_status(status);
    println!("Status atual: {:?}", status);
    status = atualizar_status(status);
    println!("Status atual: {:?}", status);
}

fn atualizar_status(status: StatusPedido)-> StatusPedido{
    if status == StatusPedido::AguardandoPagamento{
        return StatusPedido::EmProcessamento;
    }
    else if status == StatusPedido::EmProcessamento{
        return StatusPedido::Enviado;
    }
    else if status == StatusPedido::Enviado{
        return StatusPedido::Entregue;
    }
    else{
        return StatusPedido::Entregue;
    }
}