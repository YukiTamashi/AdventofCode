use std::{
    cell::RefCell,
    fs,
    rc::Rc,
    rc::Weak,
};

#[derive(Clone, Debug)]
struct Tree {
    node: Node,
    parent: Weak<RefCell<Tree>>,
}

#[derive(Clone, Debug)]
enum Node {
    File {
        name: String,
        size: i64,
    },
    Folder {
        name: String,
        size: i64,
        children: Vec<Rc<RefCell<Tree>>>,
    },
}

impl Tree {
    fn from_string(s: String) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Tree {
            node: Node::Folder {
                name: "/".to_string(),
                size: 0,
                children: vec![],
            },
            parent: Weak::new(),
        }));
        let mut current = root.clone();
        for l in s.lines() {
            let line = Line::from_string(l);
            match line {
                Line::Node(n) => {
                    if let Node::Folder{
                        children, ..
                    } = &mut (current.borrow_mut()).node
                    {
                        children.push(Rc::new(RefCell::new(Tree {
                            node: n,
                            parent: Rc::downgrade(&current),
                        })));
                    }
                }
                Line::Command(c) => match c {
                    Command::To(to) => {
                        let a = Tree::find_children(&current, &to);
                        if let Some(b) = a{
                            current = b;
                        }
                    }
                    Command::Root => current = root.clone(),
                    Command::Up => {
                        let mut b = None;
                        if let Some(a) = current.borrow().parent.upgrade() {
                            b = Some(a);
                        }
                        if let Some(c) = b {
                            current = c;
                        }
                    }
                    _ => {}
                },
            }
        }
        root
    }

    fn find_children(s: &Rc<RefCell<Self>>, to: &str) -> Option<Rc<RefCell<Self>>>{
        let mut a = None;
        if let Node::Folder {
            children, ..
        } = &(s.borrow_mut()).node
        {
            for ch in (*children).clone() {
                if let Node::Folder {
                    name, ..
                } = &ch.borrow().node
                {
                    if name == to{
                        a = Some(ch.clone())
                    }
                }
            }
        }
        a
    }

    fn set_size(&mut self) -> i64{
        match &mut self.node{
            Node::Folder {size, children, ..} => {
                *size = children.iter_mut().map(|x|x.borrow_mut().set_size() as i64).sum::<i64>();
                *size
            }
            Node::File {size, ..} => *size
        }
    }
    
    fn lower_than(&self, val: i64) -> i64{
        match &self.node{
            // ignore files, since already calculated with set_size
            Node::File {..} => {
                0
            },
            Node::Folder {size, children, ..} => {
                let csize = children.iter().map(|x| x.borrow().lower_than(val)).sum();
                // if folder size matches, add it to total
                if size <= &val{
                    csize + size
                }
                // else, return just the total
                else{
                    csize
                }
            },  
        }
    }

}

enum Line {
    Node(Node),
    Command(Command),
}

impl Line {
    fn from_string(s: &str) -> Self {
        let w: Vec<&str> = s.split_whitespace().collect();
        if w[0] == "$" {
            if w[1] == "cd" {
                match w[2] {
                    "/" => Line::Command(Command::Root),
                    ".." => Line::Command(Command::Up),
                    a => Line::Command(Command::To(a.to_owned())),
                }
            } else {
                Line::Command(Command::Skip)
            }
        } else if w[0] == "dir" {
            Line::Node(Node::Folder {
                name: w[1].to_owned(),
                size: 0,
                children: vec![],
            })
        } else {
            Line::Node(Node::File {
                name: w[1].to_owned(),
                size: w[0].parse().unwrap(),
            })
        }
    }
}

enum Command {
    Up,
    To(String),
    Root,
    Skip,
}

fn main() {
    let input = fs::read_to_string("day7.in").unwrap();
    let mut tree = Tree::from_string(input);
    tree.borrow_mut().set_size();
    println!("{tree:?}");
    let max = 100000;
    let size = tree.borrow().lower_than(max);
    println!("{size}");
}
