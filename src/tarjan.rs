use std::cmp::min;

type Set<T> = std::collections::BTreeSet<T>;


/// Tarjan's Algorithm
/// https://doi.org/10.1137/0201010
pub struct Tarjan <'tarjan, Edge, Node, Connection>
where Connection: Fn(&[Edge], &[Node], usize, usize) -> bool {
    edges: &'tarjan [Edge],
    nodes: &'tarjan [Node],
    index: usize,
    stack: Vec<usize>,
    len:usize,
    indexs:Vec<Option<usize>>,
    lowlinks:Vec<usize>,
    on_stacks:Vec<bool>,
    is_connect: Connection
}
impl <'tarjan, Edge, Node, Connection> Tarjan <'tarjan, Edge, Node, Connection> 
where Connection: Fn(&[Edge], &[Node], usize, usize) -> bool + Clone + 'tarjan{
    pub fn new(edges: &'tarjan [Edge], nodes: &'tarjan [Node], is_connect: Connection) -> Self {
        let len = nodes.len();
        Self { 
            edges,
            nodes,
            index: 0, 
            stack: vec![], 
            len, 
            indexs: vec![None; len], 
            lowlinks: vec![0; len], 
            on_stacks:  vec![false; len],
            is_connect
        }
    }
    
    fn get_connect<'a>(edges: &'tarjan [Edge], nodes: &'a [Node], is_connect: Connection ,len:usize, u: usize) -> impl Iterator<Item = usize> +'a 
    where 'tarjan: 'a{
        (0..len).into_iter().filter(move |v| is_connect(edges, nodes, u, *v))
    }
    pub fn run(&mut self) -> Vec<Set<usize>> {
        let mut scc:Vec<Set<usize>> = Vec::new();
        for u in 0..self.len {
            if self.indexs[u] == None {
                scc.append(&mut self.strongconnect(u));
            }
        }
        scc
    }
    fn strongconnect(&mut self, u: usize) -> Vec<Set<usize>> {
        let mut output = vec![];
        self.indexs[u] = Some(self.index);
        self.lowlinks[u] = self.index;
        self.index = self.index + 1;
        self.stack.push(u);
        self.on_stacks[u] = true;
        for v in Self::get_connect(self.edges, &self.nodes, self.is_connect.clone(),self.len, u) {
            let Some(v_index) = self.indexs[v] else {
                output.append(&mut self.strongconnect(v));
                self.lowlinks[u] = min(self.lowlinks[u], self.lowlinks[v]);
                continue;
            };
            if self.on_stacks[v] {
                self.lowlinks[u] = min(self.lowlinks[u], v_index);
            }
        }
        if Some(self.lowlinks[u]) == self.indexs[u] {
            let mut scc: Set<usize> = Set::new();
            while let Some(v) = self.stack.pop() {
                self.on_stacks[v] = false;
                scc.insert(v);
                if v == u {
                    break;
                }
            }
            output.push(scc);
        }
        return output;
    }   
}