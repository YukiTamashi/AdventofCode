use std::{fs, rc::Rc, borrow::BorrowMut, rc::Weak};

// Using box here instead of a weak reference is really bad, but too much hassle to deal with otherwise.
struct Tree{
    node: Node,
    parent: Option<Weak<Tree>>,
}

enum Node{
    File{name: String, size: u32},
    Folder{name: String, 
           size: u32, 
           children: Vec<Rc<Tree>>}
}

impl Tree{
    fn from_string(s: String) -> Rc<Self>{
        let mut root = Rc::new(
            Tree {  
                node: Node::Folder{
                    name: "/".to_string(), 
                    size: 0,
                    children: vec!()
                }, 
                parent: None, 
            });
        let mut current = root.clone();
        for l in s.lines(){
            let line = Line::from_string(l);
            match line{
                Line::Node(n) => {
                    if let Node::Folder {children: c, ..} = current.node.borrow_mut(){
                            c.push(Rc::new(Tree { node: n, parent: Some(Rc::downgrade(&current)) }))
                        }
                },
                Line::Command(c) =>{
                    match c{
                        Command::To(to) => todo!(),
                        Command::Root => current = root.clone(),
                        Command::Up => current = Tree::parent(&current),
                        _ => panic!()
                    }
                }
            }
        }
        root
    }

    fn parent(s: &Rc<Self>) -> Rc<Self>{
        s.parent.unwrap().upgrade().unwrap()
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
