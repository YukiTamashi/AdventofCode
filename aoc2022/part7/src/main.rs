use std::{fs, rc::Rc, rc::Weak, cell::{RefCell, UnsafeCell}, borrow::BorrowMut};

#[derive(Clone)]
struct Tree{
    node: Node,
    parent: Option<Weak<RefCell<Tree>>>,
}

#[derive(Clone)]
enum Node{
    File{name: String, size: u32},
    Folder{name: String, 
           size: u32, 
           children: Vec<Rc<RefCell<Tree>>>}
}

impl Tree{
    fn from_string(s: String) -> Self{
        let root = 
            Tree {  
                node: Node::Folder{
                    name: "/".to_string(), 
                    size: 0,
                    children: vec!()
                }, 
                parent: None, 
            };
        let mut current = Rc::new(RefCell::new(root.clone()));
        for l in s.lines(){
            let line = Line::from_string(l);
            match line{
                Line::Node(n) => {
                    if let Node::Folder {children: mut c, ..} = (current.borrow()).node.clone(){
                        c.push(Rc::new(RefCell::new(Tree { node: n, parent: Some(Rc::downgrade(&current)) })))
                        }
                },
                Line::Command(c) =>{
                    match c{
                        Command::To(to) => {if let Node::Folder {children: mut c, ..} = (current.borrow()).node.clone(){
                                                    for children in c{

                                                    }
                                                        }
                                                    },
                        Command::Root => current = Rc::new(RefCell::new(root.clone())),
                        Command::Up => {let a = current.borrow().parent.as_ref().unwrap().clone().upgrade().unwrap();
                                        current = a;},
                        _ => panic!()
                    }
                }
            }
        }
        root
    } 
}

enum Line{
    Node(Node),
    Command(Command),
}

impl Line{
    fn from_string(s: &str) -> Self{
        let w: Vec<&str> = s.split_whitespace().collect();
        if w[0] == "$"{
            if w[1] == "cd"{
                match w[2]{
                    "/" => Line::Command(Command::Root),
                    ".." => Line::Command(Command::Up),
                    a => Line::Command(Command::To(a.to_owned()))
                }
            }
            else{
                Line::Command(Command::Skip)
            }
        }
        else if w[0] == "dir"{
            Line::Node(
                Node::Folder {
                    name: w[1].to_owned(), 
                    size: 0, 
                    children: vec!()
                })
        }
        else{
            Line::Node(
                Node::File { 
                    name: w[0].to_owned(), 
                    size: w[1].parse().unwrap()
                })
        }
    }
}

enum Command{
    Up,
    To(String),
    Root,
    Skip
}

fn main() {
    let input = fs::read_to_string("day7.in").unwrap();
    let mut tree = Tree::from_string(input);
}
