// Criando nó que recebe valor T
struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>, // Opção de caixa para tornar flexível e não uma pilha.
}

// Criando a Lista Encadeada
struct LinkedList<T>{
    head: Option<Box<Node<T>>>, // Primeiro nó como cabeça.
}

// Criando métodos para a lista encadeada.
impl <T> LinkedList<T>{
    // Construtor padrão.
    fn new() -> Self{
        LinkedList{ head: None }
    }

    // Adiciona nó no final da lista encadeada.
    fn push(&mut self, value: T){
        let new_node = Box::new(Node{
            value: value,
            next: self.head.take(), // Take() fará um zig zag que faz o head ser temporariamente None para evitar ownership.
            // Isso significa que ele tira o valor e o torna None
            // para depois reutilizar o valor sem problemas.
            // Basicamente cria um novo nó como cabeça e depois conecta ambos
            // (n1)   (n2)
        });
        self.head = Some(new_node);
            // (n1)---(n2)
    }
    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node|{ // Pega o último nó e busca o substituto
            self.head = node.next; // Torna o próximo nó (substituto) na cabeça da lista
            node.value // Retorna o valor do nó que pegou
        })
    }

    fn contains(&self, value: T) -> bool where T: PartialEq,{
        let mut current = &self.head; // Começa a corrente pela cabeça
        while let Some(node) = current{ // Enquanto houver nó com valor
            if node.value == value{ // Retorna se nó buscado existe na lista
                return true;
            }
            current = &node.next; // Se n for igual o nó, passa pro próximo nó
        }
        false // Não existe o nó na lista
    }
}

fn main(){
    // O T é o tipo, i32, i64, char, &str...
    let mut list: LinkedList<i32> = LinkedList::new();

    for i in 0..500{
        list.push(i);
    }

    println!("Existe nó 20? {}", list.contains(20));
    println!("Existe nó 40? {}", list.contains(40));


    while let Some(value) = list.pop(){
        println!("Esvaziando... {}", value);
    }
}