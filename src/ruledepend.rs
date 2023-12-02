use std::{collections::{BTreeSet, HashMap}, cmp::min, usize};

use crate::syntax::{Rule, Variable};

type Set<T> = BTreeSet<T>;
type Map<K,V> = HashMap<K,V>;

fn is_connect(edges: & [Rule], nodes: & [Variable], u: usize, v:usize) -> bool {
    edges.iter().filter(|e| {e.clause == nodes[u] && e.output.data[0] == nodes[v].into()}).peekable().peek().is_some()
}

/// Tarjan's Algorithm
/// https://doi.org/10.1137/0201010
struct Tarjan <'tarjan>{
    edges: &'tarjan [Rule],
    nodes: &'tarjan [Variable],
    index: usize,
    stack: Vec<usize>,
    len:usize,
    indexs:Vec<Option<usize>>,
    lowlinks:Vec<usize>,
    on_stacks:Vec<bool>,
}
impl <'tarjan> Tarjan <'tarjan> {
    fn new(edges: &'tarjan [Rule], nodes: &'tarjan [Variable]) -> Self {
        let len = nodes.len();
        Self { 
            edges,
            nodes,
            index: 0, 
            stack: vec![], 
            len, 
            indexs: vec![None; len], 
            lowlinks: vec![0; len], 
            on_stacks:  vec![false; len]
        }
    }
    
    fn get_connect<'a>(edges: &'tarjan [Rule], nodes: &'a [Variable], len:usize, u: usize) -> impl Iterator<Item = usize> +'a 
    where 'tarjan: 'a{
        (0..len).into_iter().filter(move |v| is_connect(edges, nodes, u, *v))
    }
    fn run(&mut self) -> Vec<Set<usize>> {
        let mut scc:Vec<Set<usize>> = Vec::new();
        for u in 0..self.len {
            if self.indexs[u] == None {
                scc.push(self.strongconnect(u));
            }
        }
        scc
    }
    fn _strongconnect(&mut self, u: usize){
        self.indexs[u] = Some(self.index);
        self.lowlinks[u] = self.index;
        self.index = self.index + 1;
        self.stack.push(u);
        self.on_stacks[u] = true;
        for v in Self::get_connect(self.edges, &self.nodes, self.len, u) {
            let Some(v_index) = self.indexs[v] else {
                self._strongconnect(v);
                self.lowlinks[u] = min(self.lowlinks[u], self.lowlinks[v]);
                continue;
            };
            if self.on_stacks[v] {
                self.lowlinks[u] = min(self.lowlinks[u], v_index);
            }
        }
    }
    fn strongconnect(&mut self, u: usize) -> Set<usize> {
        self._strongconnect(u);
        let mut scc: Set<usize> = Set::new();
        while let Some(v) = self.stack.pop() {
            self.on_stacks[v] = false;
            scc.insert(v);
            if v == u {
                break;
            }
        }
        return scc;
    }   
}

fn check_edge(group1_id: &Set<usize>, group2_id: &Set<usize>, rules: &[Rule], variables: &[Variable]) -> bool{
    for i in group1_id {
        for j in group2_id {
            if is_connect(rules, variables, *i, *j) {
                return true;
            }
        }
    }
    return false;
}

fn check_edges(node_group : &[Set<usize>], rules: &[Rule], variables: &[Variable]) -> Map<usize, Set<usize>> {
    let mut edge: Map<usize, Set<usize>> = Map::new();
    for i in 0..node_group.len() {
        for j in 0..node_group.len() {
            if i == j  {continue;}
            if check_edge(&node_group[i], &node_group[j], rules, variables) {
                // edge.insert(i, j);
                edge.entry(i).or_default().insert(j);
            }
        }
    }
    edge
}

pub struct RuleGraph{
    node_group: Vec<Set<usize>>,
    rules: Vec<Rule>,
    indexing: Map<Variable, usize>,
    variables: Vec<Variable>,
    edge: Map<usize, Set<usize>>
}
impl RuleGraph {
    pub fn new(rules: Vec<Rule>) -> Self {
        let variables:Set<Variable> = rules.iter().map(|rule| rule.clause).collect();
        let variables:Vec<Variable> = variables.iter().map(|x| *x).collect();
        let node_group = Tarjan::new(&rules, &variables).run();
        println!("{:?} {:?}", variables, node_group);

        let indexing = node_group.iter().enumerate().fold(Map::new(), |mut map: Map<Variable, usize>, (id, variable_set)|{
            variable_set.iter().for_each(|variable_id| {map.insert(variables[*variable_id],  id);});
            map
        });
        let edge = check_edges(&node_group, &rules, &variables);
        Self {
            node_group, variables, indexing, rules, edge
        }
    }
    pub fn get(&self, variable: Variable) -> Vec<Variable> {
        let mut nodes:Vec<usize> = vec![self.indexing[&variable]];
        let mut checked: Set<usize> = Set::new();
        let mut output: Vec<Variable> = vec![];
        while let Some(node) = nodes.pop() {
            if checked.contains(&node) {continue;}
            checked.insert(node);
            for v_id in &self.node_group[node] {
                output.push(self.variables[*v_id]);
            }
            let nexts = self.edge.get(&node).unwrap();
            for next in nexts {
                nodes.push(*next);
            }
        }
        output
    }

    pub fn gets_var(&self, variable: impl Iterator<Item = Variable>) -> Vec<Variable> {
        let mut nodes:Vec<usize> = variable.map(|var| self.indexing[&var]).collect();
        let mut checked: Set<usize> = Set::new();
        let mut output: Vec<Variable> = vec![];
        while let Some(node) = nodes.pop() {
            if checked.contains(&node) {continue;}
            checked.insert(node);
            for v_id in &self.node_group[node] {
                output.push(self.variables[*v_id]);
            }
            let Some(nexts) = self.edge.get(&node) else {
                continue;
            };
            for next in nexts {
                nodes.push(*next);
            }
        }
        output
    }

    pub fn gets_rule(&self, variable: impl Iterator<Item = Variable>) -> Vec<usize> {
        let mut nodes:Vec<usize> = variable.map(|var| self.indexing[&var]).collect();
        let mut checked: Set<usize> = Set::new();
        let mut output: Vec<usize> = vec![];
        while let Some(node) = nodes.pop() {
            if checked.contains(&node) {continue;}
            checked.insert(node);
            for v_id in &self.node_group[node] {
                self.rules.iter().enumerate()
                    .filter(|(_, rule)| {rule.clause == self.variables[*v_id]})
                    .for_each(|(id, _)| output.push(id))
            }
            let Some(nexts) = self.edge.get(&node) else {
                continue;
            };
            for next in nexts {
                nodes.push(*next);
            }
        }
        output
    }

}
