// bstree.rs - BSTree(BinarySearchTree)
// The goal: implementing binary-search tree. check the removed items are
// correctly deallocated from memory
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
use std::cell::{RefCell, RefMut};

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    key: T,
}

impl<T> Node<T> {
    fn new(key: T) -> Node<T> {
        Node::<T> {
            left: None,
            right: None,
            key: key,
        }
    }

    fn left_empty(&self) -> bool {
        match self.left {
            Some(_) => false,
            None => true,
        }
    }

    fn right_empty(&self) -> bool {
        match self.right {
            Some(_) => false,
            None => true,
        }
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }
}

struct BSTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> BSTree<T> {
    fn new() -> BSTree<T> {
        BSTree::<T> {
            root: None,
        }
    }

    fn insert(&mut self, key: T) {
        println!("inserting...");
        if self.root.is_some() {
            println!("root is some!");
            let mut node_ptr = &mut self.root as *mut Option<Box<Node<T>>>;
            let mut parent_ptr = &mut self.root as *mut Option<Box<Node<T>>>;
            unsafe {
                loop {
                    match *node_ptr {
                        Some(ref mut node) => {
                            parent_ptr = node_ptr;
                            if key < node.key {
                                println!("go left");
                                node_ptr = &mut node.left as *mut Option<Box<Node<T>>>;;
                            } else {
                                println!("go right");
                                node_ptr = &mut node.right as *mut Option<Box<Node<T>>>;;
                            }
                        },
                        None => { break; },
                    }
                }
                match *parent_ptr {
                    Some(ref mut node) => {
                        if key < node.key {
                            println!("make new node to left");
                            node.left = Some(Box::new(Node::new(key)));
                        } else {
                            println!("make new node to right");
                            node.right = Some(Box::new(Node::new(key)));
                        }
                    },
                    None => {},
                }
            }
        } else {
            println!("root is none");
            /*
            // how below code not crash?
            // root is None so node has not member left or right!
            self.root.as_mut().map(|node| {
                if key < node.key {
                    node.left = Some(Box::new(Node::new(key)));
                } else {
                    node.right = Some(Box::new(Node::new(key)));
                }
            });
            */
            self.root = Some(Box::new(Node::new(key)));
        }
    }

    fn search(&self, key: &T) -> Option<&Node<T>> {
        let mut find = &self.root;
        while match *find {
            Some(_) => true,
            None => false,
        } {
            match *find {
                Some(ref n) => {
                    if *key == n.key {
                        break;
                    } else if *key < n.key {
                        find = &n.left;
                    } else if *key > n.key {
                        find = &n.right;
                    }
                },
                None => {},
            }
        }
        match *find {
            Some(ref n) => { Some(n) },
            None => { None },
        }
    }
}

fn main() {
    let mut tree = BSTree::<i32>::new();
    println!("---- insertion ----");
    tree.insert(5);
    tree.insert(10);
    tree.insert(3);
    tree.insert(20);

    println!("---- searching ----");
    let f = tree.search(&20);
    println!("{} == `20`", &f.unwrap().key);

    let f = tree.search(&5);
    println!("root should 5, {}", &f.unwrap().key);

    match f.unwrap().right {
        Some(ref node) => {
            println!("right of root should 10, {}", node.key);
        },
        None => {
        },
    }

    println!("value 7 is not in the tree. search 7");
    let f = tree.search(&7);
    match f {
        Some(_) => {
            println!("something found!");
        },
        None => {
            println!("nothing found.");
        },
    }
}
