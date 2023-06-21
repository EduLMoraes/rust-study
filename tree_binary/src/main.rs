
// Aqui criamos um nó de uma árvore binária, que pode
// ter ramificações tanto pra esquerda quanto para direita,
// essas ramificações são outras nós de árvores.
struct TreeNode<T>{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// Aqui criamos a raiz da árvore, o nó acima de todos,
// essa raiz recebe um nó que pode ter N ramificações.
struct BinaryTree<T>{
    root: Option<Box<TreeNode<T>>>,
}

impl <T: std::cmp::PartialOrd + std::fmt::Debug> BinaryTree<T> {
    // Construtor da árvore binária, para começar com
    // o valor None atribuímos a raiz como None.
    fn new() -> Self{
        BinaryTree{ root:None }
    }

    // Aqui para inserir nós passamos por todos os outros nós
    // com a ajuda da função repercussiva insert_node()
    fn insert(&mut self, value: T){
        self.root = Self::insert_node(self.root.take(), value);
    }
    
    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>>{
        match node{
            // Se o valor inserido for maior que o valor do nó
            // ele irá para a direita, caso contrário irá para
            // a esquerda do nó.
            Some(mut n) =>{
                if value < n.value{
                    n.left = Self::insert_node(n.left.take(), value);
                }
                else{
                    n.right = Self::insert_node(n.right.take(), value);
                }
                Some(n)
            }
            // Após ele colocar o último nó ele atribui um último nó,
            // esse último nó atribuído será None, por isso criamos
            // o None com o valor e suas ramificações como None.
            None => Some(Box::new(TreeNode{
                value,
                left: None, 
                right: None,
            })),
        }
    }
    
    // Aqui ele exibe a árvore em ordem transversal,
    // ou seja, ele passa pelo último nó a esquerda,
    // volta pro nó raiz e vai para o da direita
    fn inorder_traversal(&self){
        Self::inorder(self.root.as_ref());
    }

    // Aqui ocorre a exibição.
    fn inorder(node: Option<&Box<TreeNode<T>>>){
        if let Some(n) = node{
            Self::inorder(n.left.as_ref());
            println!("{:?}", n.value);
            Self::inorder(n.right.as_ref());
        }
    }
}

fn main() {
    let mut tree: BinaryTree<i32> = BinaryTree::new();
    tree.insert(5); //                (5)
    tree.insert(3); //               /   \ 
    tree.insert(8); //            (3)     (8)
    tree.insert(2); //            / \     / \
    tree.insert(4); //          (2) (4) (7) (9)
    tree.insert(7); 
    tree.insert(9);           

    tree.inorder_traversal();
}
