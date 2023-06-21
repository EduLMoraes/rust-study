#[derive(PartialEq)]
struct Node<T>{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct BinaryTree<T>{
    root: Option<Box<Node<T>>>
}

impl <T: std::cmp::PartialOrd + std::fmt::Debug> 
BinaryTree<T>{

    fn new() -> Self {
        BinaryTree {root: None}
    }

    fn insert(&mut self, value: T){
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>>{
        match node{
            Some(mut n) => {
                if value < n.value{
                    n.left = Self::insert_node(n.left.take(), value);
                }
                else{
                    n.right = Self::insert_node(n.right.take(), value);
                }
                Some(n)
            }
            None => Some(Box::new(Node {
                value: value, 
                left: None, 
                right: None, 
            })),
        }
    }

    fn inorder_traversal(&self){
        Self::inorder(self.root.as_ref());
    }

    fn inorder(node: Option<&Box<Node<T>>>){
        if let Some(n) = node{
            Self::inorder(n.left.as_ref());
            print!("{:?}", n.value);
            Self::inorder(n.right.as_ref());
        }
    }

    /*
        Exercício 1:
        Implemente o método contains(&self, value: T) -> bool na struct BinaryTree<T>, 
        que verifica se um determinado valor está presente na árvore. Retorne true se
        o valor estiver na árvore e false caso contrário.
    */

    fn contains(&self, value: T) -> bool {
        return Self::_contains(self.root.as_ref(), &value);
    }

    fn _contains(node: Option<&Box<Node<T>>>, value: &T) -> bool{
        if let Some(n) = node{
            Self::_contains(n.left.as_ref(), &value);

            if &&n.value == &value{
                return true;
            }

            Self::_contains(n.right.as_ref(), &value);
        }
        
        return false;
    } 

    /* 
        Exercício 2:
        Implemente o método height(&self) -> i32 na struct BinaryTree<T>, que retorna 
        a altura da árvore. A altura de uma árvore é definida como o número máximo de
        arestas entre a raiz e uma folha. Considere que uma árvore vazia tem altura -1.
    */

    fn height(&self) -> i32 {
        if Some(&self.root) == Some(&None){
            return -1;
        }

        let mut node = &self.root;
        let mut count_left = 0;
        let mut count_right = 0;

        while let Some(n) = node{
            count_left +=1;
            if Some(&n.left) != Some(&None){
                node = &n.left;
            }
            else{
                node = &n.right;
            }
        }

        node = &self.root;

        while let Some(n) = node{
            count_right +=1;
            if Some(&n.right) != Some(&None){
                node = &n.right;
            }
            else{
                node = &n.left;
            }
        }

        if count_left > count_right {
            return count_left;
        }
        else{
            return count_right;
        }
    }

    /* 
        Exercício 3:
        Implemente o método is_balanced(&self) -> bool na struct BinaryTree<T>, que 
        verifica se a árvore está balanceada. Uma árvore está balanceada se a diferença 
        de altura entre suas subárvores esquerda e direita é no máximo 1.
    */

    fn is_balanced(&self) -> bool {
        if Some(&self.root) == Some(&None){
            return true;
        }

        let mut node = &self.root;
        let mut count_left = 0;
        let mut count_right = 0;

        while let Some(n) = node{
            count_left +=1;
            if Some(&n.left) != Some(&None){
                node = &n.left;
            }
            else{
                node = &n.right;
            }
        }

        node = &self.root;

        while let Some(n) = node{
            count_right +=1;
            if Some(&n.right) != Some(&None){
                node = &n.right;
            }
            else{
                node = &n.left;
            }
        }

        if count_left == count_right || count_left+1 == count_right || count_left == count_right+1{
            return true;
        }
        else{
            return false;
        }
    }

    /* 
        Exercício 4:
        Implemente o método preorder_traversal(&self) na struct BinaryTree<T>, que realiza
        uma travessia em PRÉ-ordem na árvore e imprime os valores dos nós.
    */

    fn preorder_traversal(&self){
        Self::preorder(self.root.as_ref());
    }

    fn preorder(node: Option<&Box<Node<T>>>){
        if let Some(n) = node{
            print!("{:?}", n.value);
            Self::preorder(n.left.as_ref());
            Self::preorder(n.right.as_ref());
        }
    }


    /* 
        Exercício 5:
        Implemente o método postorder_traversal(&self) na struct BinaryTree<T>, que realiza 
        uma travessia em PÓS-ordem na árvore e imprime os valores dos nós.
    */

    fn postorder_traversal(&self){
        Self::postorder(self.root.as_ref());
    }

    fn postorder(node: Option<&Box<Node<T>>>){
        if let Some(n) = node{
            Self::postorder(n.left.as_ref());
            Self::postorder(n.right.as_ref());
            print!("{:?}", n.value);
        }
    }
}

fn main(){
    let mut tree: BinaryTree<i32> = BinaryTree::new();
    tree.insert(5); //                (5)
    tree.insert(3); //               /   \ 
    tree.insert(8); //            (3)     (8)
    tree.insert(2); //            / \     / \
    tree.insert(4); //          (2) (4) (7) (9)
    tree.insert(7); 
    tree.insert(9);           

    tree.inorder_traversal();
    print!("\n");
    tree.preorder_traversal();
    print!("\n");
    tree.postorder_traversal();
    print!("\n");

    println!("Height: {}", tree.height());
    println!("Contains: {}", tree.contains(5));
    println!("Contains: {}", tree.contains(20));
    println!("is_balanced: {}", tree.is_balanced());

}