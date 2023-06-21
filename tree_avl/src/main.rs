#[derive(Debug)]

// Como já vimos na árvore binária, aqui é o a estrutura do nó.
struct Node<T>{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: i32,
}

impl<T> Node<T>{
    fn new(value:T)-> Self{
        Node{
            value: value,
            left: None,
            right: None,
            height: 0,
        }
    }
}

struct AvlTree<T>{
    root: Option<Box<Node<T>>>,
}

impl <T: Ord> AvlTree<T>{
    fn new() -> Self{
        AvlTree{ root: None }
    }
    
    fn insert(&mut self, value: T){
        let root = self.root.take();
        self.root = self.insert_node(root, value);
    }

    // O mesmo da árvore binária, a diferença está no balance.
    // ele retorna o valor inserido após balancear a árvore.
    fn insert_node(&mut self, node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>>{
        match node{
            Some(mut n) =>{
                if value< n.value{
                    n.left = self.insert_node(n.left.take(), value);
                }
                else{
                    n.right = self.insert_node(n.right.take(), value);
                }
                n.height = 1 + std::cmp::max(self.get_height(&n.left), self.get_height(&n.right));
                Some(self.balance(n))
            }
            None => Some(Box::new(Node::new(value))),
        }
    }

    // O balance verificará para qual lado está desequilibrado,
    // com isto, ele decide para qual lado será feita a rotação.
    fn balance(&self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let balance_factor = self.get_balance_factor(&node);
        
        if balance_factor > 1 {
            if self.get_balance_factor(&node.left.as_ref().unwrap()) < 0 {
                node.left = Some(self.left_rotate(node.left.take().unwrap()));
            }
            return self.right_rotate(node);
        }
        
        if balance_factor < -1 {
            if self.get_balance_factor(&node.right.as_ref().unwrap()) > 0 {
                node.right = Some(self.right_rotate(node.right.take().unwrap()));
            }
            return self.left_rotate(node);
        }

        node
    }

    // Aqui ambas as funções funcionam do mesmo jeito, porém para
    // lados opostos.
    fn left_rotate(&self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        // Pega o valor da direita do nó e o passa como nova raiz
        let mut new_root = node.right.take().unwrap();
        // Pega temporariamente o valor da esquerda do nó.
        let temp = new_root.left.take();

        // Passa o valor temporário que estava à esquerda
        // para a direita.
        node.right = temp;
        
        // Renova a altura da árvore.
        self.update_height(&mut node);
        self.update_height(&mut new_root);
        
        new_root.left = Some(node); // Define a nova raiz como o valor a esquerda do nó.
        // Feito a modificação da raiz, agora é só retornar.
        new_root
        
    }

    fn right_rotate(&self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.left.take().unwrap();
        let temp = new_root.right.take();

        node.left = temp;
        
        self.update_height(&mut node);
        self.update_height(&mut new_root);
        
        new_root.right = Some(node); 
        new_root
    }

    // Pega o valor da altura da árvore e o retorna, se o nó
    // for none ele retorna -1.
    fn get_height(&self, node: &Option<Box<Node<T>>>) -> i32{
        match node{
            Some(n) => n.height,
            None => -1,
        }
    }
    
    fn get_balance_factor(&self, node: &Box<Node<T>>) -> i32{
        self.get_height(&node.left) - self.get_height(&node.right)
    }

    fn update_height(&self, node: &mut Box<Node<T>>){
        node.height = 1 + std::cmp::max(self.get_height(&node.left), self.get_height(&node.right));
    }
}

fn main(){
    let mut avl_tree: AvlTree<i32> = AvlTree::new();

    avl_tree.insert(10);    //          40    
    avl_tree.insert(20);    //         /   \
    avl_tree.insert(30);    //       20     50
    avl_tree.insert(40);    //      /  \      \
    avl_tree.insert(50);    //    10    30     60
    avl_tree.insert(60);

    println!("{:?}\n", avl_tree.root);
}