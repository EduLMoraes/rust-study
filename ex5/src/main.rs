#[derive(Debug)]
struct Node<T>
{
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct List<T>{
    head: Option<Box<Node<T>>>,
}

impl<T> List<T>{
    fn new() -> Self{
        List{ head: None }
    }

    fn push(&mut self, value: T){
        let new_node = Box::new(Node{
            value: value,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            node.value
        })
    }

    fn contains(&self, value: T) -> bool where T: PartialEq,{
        let mut current = &self.head;

        while let Some(node) = current{
            if node.value == value{
                return true;
            }
            current = &node.next;
        }
        return false;
    }

    fn len(&self) -> usize{
        let mut current = &self.head;
        let mut cont: usize = 0;
        
        while let Some(node) = current{
            cont += 1;
            current = &node.next;
        }
        return cont;
    }

    fn get(&self, value: usize) -> &T{
        let mut current = &self.head;
        let mut cont: usize = 0;

        while let Some(node) = current{
            if cont == value{
                return Some(&node.value).unwrap();
            }
            cont += 1;
            current = &node.next;
        }

        return None.unwrap();
    }

    fn reverse(&self)-> List<&T>{
        let mut inverse = List::new();
        
        for i in 0..self.len(){
            let node = self.get(i);
            inverse.push(node);
        }

        return inverse;
    }

    /*
    Exercício 1:
        Implemente o método remove(&mut self, value: T) na struct List<T>,
        que remove a primeira ocorrência do valor especificado da lista encadeada.
    */
    fn remove(&self, value: T) -> List<&T> where T: PartialEq, {
        let mut removed: List<&T> = List::new();
        
        for i in 0..self.len(){
            let node = self.get(i);

            if node != &value{
                removed.push(node);
            }
        }

        return removed;
    }

    /*
    Exercício 2:
        Implemente o método is_empty(&self) -> bool na struct List<T>,
        que retorna um valor booleano indicando se a lista encadeada 
        está vazia ou não.
    */

    fn is_empty(&self) -> bool{
        if self.len() == 0{
            return true;
        }

        return false;
    }

    /*
    Exercício 3:
        Implemente o método clear(&mut self) na struct List<T>, que 
        remove todos os elementos da lista encadeada, deixando-a vazia.
    */
    fn clear(&mut self){
        
        self.head.take().map(|_node: Box<Node<T>>|{
            self.head = None;
        });
    }

    /*
    Exercício 4:
        Implemente o método iter(&self) -> Iter<'_, T> na struct List<T>, 
        que retorna um iterador sobre os elementos da lista encadeada.
    
    fn iter(&self) -> Iter<'_, &T>{
        let mut vet: Vec<&T> = Vec::new();
        
        for i in 0..self.len() {
            vet.push(self.get(i));
        }
        
        let vet = vet.iter();
        
        return vet;
    }
    */

    /*
    Exercício 5:
        Implemente o método find(&self, value: T) -> Option<usize> na struct List<T>, 
        que retorna a posição (índice) da primeira ocorrência do valor especificado na 
        lista encadeada. Se o valor não for encontrado, retorne None.
    */

    fn find(&self, value: T) -> Option<usize> where T: PartialEq,{
        let mut current = &self.head;
        let mut cont: usize = 0;

        while let Some(node) = current{
            if value == node.value{
                return Some(cont);
            }
            cont += 1;
            current = &node.next;
        }

        return None.unwrap();
    }

}

fn main(){
    // O T é o tipo, i32, i64, char, &str...
    let mut list: List<i32> = List::new();
    println!("A nova lista está vazia? {}", list.is_empty());

    for i in 0..20{
        list.push(i);
    }

    
    println!("Existe nó 20? {}", list.contains(20));
    println!("Existe nó 40? {}", list.contains(40));
    println!("Nó na posição 5 é {:?}", list.get(5));
    println!("O Tamanho da lista é {:#?}", list.len());
    println!("Lista normal\n {:?}\n", list);
    println!("Lista invertida\n {:?}\n", list.reverse());
    println!("Lista sem o 15\n {:?}", list.remove(15));
    println!("Onde está o número 5? {:?}", list.find(5));
    println!("Lista é vazia? {:?}", list.is_empty());
    println!("Removendo com pop {:?}", list.pop());
    list.clear();
    println!("Removendo tudo {:?}", list);

    

}