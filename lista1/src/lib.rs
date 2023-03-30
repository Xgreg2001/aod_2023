use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

pub struct Node<T> {
    pub index: usize,
    pub data: Option<T>,
    pub edges: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(index: usize) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            index,
            data: None,
            edges: Vec::new(),
        }))
    }
}

pub struct Graph<T> {
    n: usize,
    nodes: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Graph<T> {
    fn new(n: usize) -> Graph<T> {
        let mut g = Graph {
            n,
            nodes: Vec::new(),
        };

        for i in 0..n {
            g.nodes.push(Node::new(i));
        }

        g
    }

    pub fn get_n(&self) -> usize {
        self.n
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        let u = self.nodes.get(u).unwrap();
        let v = self.nodes.get(v).unwrap();
        let mut u_mut = u.borrow_mut();
        u_mut.edges.push(v.clone());
    }

    pub fn get_node(&self, index: usize) -> Rc<RefCell<Node<T>>> {
        self.nodes.get(index).unwrap().clone()
    }

    pub fn add_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.nodes.push(node);
        self.n += 1;
    }

    pub fn dfs_form_node(&self, root: Rc<RefCell<Node<T>>>, f: &mut dyn FnMut(&Node<T>)) {
        let mut visited = vec![false; self.n];

        let mut stack = vec![root];

        while !stack.is_empty() {
            let u = stack.pop().unwrap();

            let u_mut = u.borrow_mut();

            if !visited[u_mut.index] {
                f(&u_mut);
                visited[u_mut.index] = true;
            }

            for v in u_mut.edges.iter() {
                let v_mut = v.borrow_mut();
                if !visited[v_mut.index] {
                    stack.push(v.clone());
                }
            }
        }
    }

    pub fn dfs(&self, f: &mut dyn FnMut(&Node<T>)) {
        let mut visited = vec![false; self.n];

        for i in 0..self.n {
            if !visited[i] {
                let root = self.get_node(i);
                let mut stack = vec![root];

                while !stack.is_empty() {
                    let u = stack.pop().unwrap();

                    let u_mut = u.borrow_mut();

                    if !visited[u_mut.index] {
                        f(&u_mut);
                        visited[u_mut.index] = true;
                    }

                    for v in u_mut.edges.iter() {
                        let v_mut = v.borrow_mut();
                        if !visited[v_mut.index] {
                            stack.push(v.clone());
                        }
                    }
                }
            }
        }
    }

    pub fn dfs_with_tree(&self, f: &mut dyn FnMut(&Node<T>)) -> Graph<T> {
        let mut visited = vec![false; self.n];
        let mut tree = Graph::new(self.n);

        for i in 0..self.n {
            if !visited[i] {
                let root = self.get_node(i);
                let mut stack = vec![root];

                while !stack.is_empty() {
                    let u = stack.pop().unwrap();

                    let u_mut = u.borrow_mut();

                    if !visited[u_mut.index] {
                        f(&u_mut);
                        visited[u_mut.index] = true;
                    }

                    for v in u_mut.edges.iter() {
                        let v_mut = v.borrow_mut();
                        if !visited[v_mut.index] {
                            stack.push(v.clone());
                            tree.add_edge(u_mut.index, v_mut.index);
                        }
                    }
                }
            }
        }

        tree
    }

    pub fn build_from_file(file_path: &str) -> Result<Graph<T>, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let directed = match lines.next() {
            Some("D") => true,
            Some("U") => false,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid graph type")),
        };

        let n = lines.next().unwrap().parse::<usize>().unwrap();
        let m = lines.next().unwrap().parse::<usize>().unwrap();

        let mut graph = Graph::new(n);

        for _ in 0..m {
            let line = lines.next().unwrap();
            let mut line = line.split_whitespace();
            let u = line.next().unwrap().parse::<usize>().unwrap() - 1;
            let v = line.next().unwrap().parse::<usize>().unwrap() - 1;

            graph.add_edge(u, v);
            if !directed {
                graph.add_edge(v, u);
            }
        }

        Ok(graph)
    }

    pub fn bfs_from_node(&self, root: Rc<RefCell<Node<T>>>, f: &mut dyn FnMut(&Node<T>)) {
        let mut visited = vec![false; self.n];

        let mut queue = vec![root];

        while !queue.is_empty() {
            let u = queue.remove(0);

            let u_mut = u.borrow_mut();

            if !visited[u_mut.index] {
                f(&u_mut);
                visited[u_mut.index] = true;
            }

            for v in u_mut.edges.iter() {
                let v_mut = v.borrow_mut();
                if !visited[v_mut.index] {
                    queue.push(v.clone());
                }
            }
        }
    }

    pub fn bfs(&self, f: &mut dyn FnMut(&Node<T>)) {
        let mut visited = vec![false; self.n];
        let mut grey = vec![false; self.n];

        for i in 0..self.n {
            if !visited[i] {
                let root = self.get_node(i);
                let mut queue = VecDeque::new();
                queue.push_back(root);

                while !queue.is_empty() {
                    let u = queue.pop_front().unwrap();

                    let u_mut = u.borrow_mut();

                    if !visited[u_mut.index] {
                        f(&u_mut);
                        visited[u_mut.index] = true;
                    }

                    for v in u_mut.edges.iter() {
                        let v_mut = v.borrow_mut();
                        if !visited[v_mut.index] && !grey[v_mut.index] {
                            grey[v_mut.index] = true;
                            queue.push_back(v.clone());
                        }
                    }
                }
            }
        }
    }

    pub fn bfs_with_tree(&self, f: &mut dyn FnMut(&Node<T>)) -> Graph<T> {
        let mut visited = vec![false; self.n];
        let mut grey = vec![false; self.n];

        let mut tree = Graph::new(self.n);

        for i in 0..self.n {
            if !visited[i] {
                let root = self.get_node(i);
                let mut queue = VecDeque::new();
                queue.push_back(root);

                while !queue.is_empty() {
                    let u = queue.pop_front().unwrap();

                    let u_mut = u.borrow_mut();

                    if !visited[u_mut.index] {
                        f(&u_mut);
                        visited[u_mut.index] = true;
                    }

                    for v in u_mut.edges.iter() {
                        let v_mut = v.borrow_mut();
                        if !visited[v_mut.index] && !grey[v_mut.index] {
                            grey[v_mut.index] = true;
                            queue.push_back(v.clone());
                            tree.add_edge(u_mut.index, v_mut.index);
                        }
                    }
                }
            }
        }

        tree
    }

    pub fn topological_sort(&mut self) -> Option<Vec<usize>> {
        let mut indegree = vec![0; self.n];
        let mut order = vec![0; self.n];
        let n = self.n;

        for u in self.nodes.iter() {
            let u = u.borrow();
            for v in u.edges.iter() {
                let v = v.borrow();
                indegree[v.index] += 1;
            }
        }

        let mut list = Vec::new();
        let mut next = 0;

        indegree.iter().enumerate().for_each(|(i, &x)| {
            if x == 0 {
                list.push(i);
            }
        });

        while !list.is_empty() {
            let u = list.pop().unwrap();
            let node = self.get_node(u);
            next += 1;
            order[u] = next;

            let node = node.borrow();

            for v in node.edges.iter() {
                let v = v.borrow();
                indegree[v.index] -= 1;
                if indegree[v.index] == 0 {
                    list.push(v.index);
                }
            }
        }
        if next < n {
            None
        } else {
            Some(order)
        }
    }

    pub fn find_strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut stack = Vec::new();
        let mut components = Vec::new();

        for i in 0..self.n {
            if visited[i] {
                continue;
            }
            self.fill_order(i, &mut visited, &mut stack);
        }

        let gr = self.transpose();

        visited = vec![false; self.n];

        while !stack.is_empty() {
            let u = stack.pop().unwrap();

            if visited[u] {
                continue;
            }
            components.push(gr.dfs_component_util(u, &mut visited));
        }

        components
    }

    fn fill_order(&self, u: usize, visited: &mut [bool], stack: &mut Vec<usize>) {
        let mut stack_local = Vec::new();
        let mut current = u;
        visited[current] = true;
        loop {
            let node = self.get_node(current);
            let node = node.borrow();
            let mut all_visited = true;
            for v in node.edges.iter() {
                let v = v.borrow();
                if !visited[v.index] {
                    stack_local.push(current);
                    current = v.index;
                    visited[current] = true;
                    all_visited = false;
                    break;
                }
            }
            if all_visited {
                stack.push(current);
                if let Some(next) = stack_local.pop() {
                    current = next;
                } else {
                    break;
                }
            }
        }
    }

    fn transpose(&self) -> Graph<T> {
        let mut gr = Graph::new(self.n);
        for i in 0..self.n {
            let node = self.get_node(i);
            let node = node.borrow();
            for v in node.edges.iter() {
                let v = v.borrow();
                gr.add_edge(v.index, node.index);
            }
        }
        gr
    }

    fn dfs_component_util(&self, u: usize, visited: &mut [bool]) -> Vec<usize> {
        let mut component = Vec::new();
        let mut stack = Vec::new();
        stack.push(u);

        while !stack.is_empty() {
            let u = stack.pop().unwrap();

            if visited[u] {
                continue;
            }

            visited[u] = true;
            component.push(u);

            let node = self.get_node(u);
            let node = node.borrow();

            for v in node.edges.iter() {
                let v = v.borrow();
                if !visited[v.index] {
                    stack.push(v.index);
                }
            }
        }

        component
    }

    pub fn get_bipartition(&self) -> Option<(Vec<usize>, Vec<usize>)> {
        let mut visited = vec![false; self.n];
        let mut color = vec![0; self.n];

        for i in 0..self.n {
            if !visited[i] && !self.bfs_bipartition(i, &mut visited, &mut color) {
                return None;
            }
        }

        let mut a = Vec::new();
        let mut b = Vec::new();

        color.iter().enumerate().for_each(|(i, &x)| {
            if x == 1 {
                a.push(i);
            } else {
                b.push(i);
            }
        });

        Some((a, b))
    }

    fn bfs_bipartition(&self, i: usize, visited: &mut [bool], color: &mut [i32]) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(i);
        visited[i] = true;
        color[i] = 1;

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();

            let node = self.get_node(u);
            let node = node.borrow();

            for v in node.edges.iter() {
                let v = v.borrow();
                if !visited[v.index] {
                    visited[v.index] = true;
                    color[v.index] = -color[u];
                    queue.push_back(v.index);
                } else if color[v.index] == color[u] {
                    return false;
                }
            }
        }

        true
    }
}
