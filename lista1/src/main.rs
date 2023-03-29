use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

struct Data {
    index: usize,
    color: Color,
    pi: Option<usize>,
    d: Option<usize>,
}

struct Node {
    data: Data,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(index: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: Data {
                index,
                color: Color::White,
                pi: None,
                d: None,
            },
            edges: Vec::new(),
        }))
    }
}

fn init() -> Rc<RefCell<Node>> {
    let root = Node::new(1);

    let b = Node::new(2);
    let c = Node::new(3);
    let d = Node::new(4);
    let e = Node::new(5);
    let f = Node::new(6);

    {
        let mut mut_root = root.borrow_mut();
        mut_root.edges.push(b);
        mut_root.edges.push(c.clone());
        mut_root.edges.push(d);

        let mut mut_c = c.borrow_mut();
        mut_c.edges.push(e);
        mut_c.edges.push(f);
        mut_c.edges.push(root.clone());
    }

    root
}

fn bfs(root: Rc<RefCell<Node>>, f: &dyn Fn(&Data)) {
    {
        let mut s = root.borrow_mut();
        s.data.color = Color::Gray;
        s.data.d = Some(0);
    }

    let mut queue = vec![root];
    while !queue.is_empty() {
        let u = queue.remove(0);
        let mut u = u.borrow_mut();
        f(&u.data);

        for v in u.edges.iter() {
            let mut v_mut = v.borrow_mut();
            if v_mut.data.color == Color::White {
                v_mut.data.color = Color::Gray;
                v_mut.data.d = Some(
                    u.data
                        .d
                        .expect("we should only operate on previously visited nodes!")
                        + 1,
                );
                v_mut.data.pi = Some(u.data.index);
                queue.push(v.clone());
            }
        }
        u.data.color = Color::Black;
    }
}

fn main() {
    let root = init();

    let f = |data: &Data| {
        println!(
            "index: {}, color: {:?}, pi: {:?}, d: {:?}",
            data.index, data.color, data.pi, data.d
        );
    };

    bfs(root, &f);
}
