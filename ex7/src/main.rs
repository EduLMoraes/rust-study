#[derive(Debug)]

struct Node<T>{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: i32,
}

impl <T> Node<T> {
    fn new(value: T) -> Self{
        Node{
            value: value,
            left: None,
            right: None,
            height: 0,
        }
    }
}

struct AVLTree<T>{
    root: Option<Box<Node<T>>>,
}

impl <T> AVLTree<T> {
    fn new() - Self{
        AVLTree{ root: None }
    }

    fn insert(&mut self, value: T){
        let root =  self.root.take();
        self.root = self.insert_node(root, value);
    }

    fn insert_node(&mut self, node: Option<Box<Node<T>>>, value: T)
    -> Option<Box<Node<T>>{

        match node{
            Some(mut n) => {
                if value < n.value{
                    n.left = self.insert_node(n.left.take(), value);
                }
                else{
                    n.right = self.insert_node(n.right.take(), value);
                }
                n.height = 1 + std::cmp::max(self.get_height(&n.left), self.get_height(&n.right));
                Node => Some(Box::new(Node::new(value))),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
