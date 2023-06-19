
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

    fn get(&self, value: usize) -> Option<&T>{
        let mut current = &self.head;
        let mut cont: usize = 0;

        while let Some(node) = current{
            if cont == value{
                return Some(&node.value);
            }
            cont += 1;
            current = &node.next;
        }

        return None;
    }

    fn reverse(&self)-> List<Option<&T>>{
        let mut inverse = List::new();
        
        for i in 0..self.len(){
            let node = self.get(i);
            inverse.push(node);
        }

        return inverse;
    }
}

fn main(){
    // O T é o tipo, i32, i64, char, &str...
    let mut list: List<i32> = List::new();

    for i in 0..20{
        list.push(i);
    }

    
    println!("Existe nó 20? {}", list.contains(20));
    println!("Existe nó 40? {}", list.contains(40));
    println!("Nó na posição 5 é {:?}", list.get(5));
    println!("O Tamanho da lista é {:#?}", list.len());
    println!("Lista normal\n {:?}\n", list);
    println!("Lista invertida\n {:?}", list.reverse());
    

}