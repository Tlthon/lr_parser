use std::{collections::{BTreeSet, HashMap}, usize};

use once_cell::sync::Lazy;

use crate::{syntax::{Rule, Variable}, tarjan::Tarjan};

type Set<T> = BTreeSet<T>;
type Map<K,V> = HashMap<K,V>;

fn is_connect(edges: & [Rule], nodes: & [Variable], u: usize, v:usize) -> bool {
    edges.iter().filter(
        |e| {
            e.clause == nodes[u] && e.output.data.get(0) == Some(&nodes[v].into())
        })
        .peekable().peek().is_some()
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
static EMPTY_USIZE:Lazy<Set<usize>> = Lazy::new(|| Set::default());

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
        let node_group = Tarjan::new(&rules, &variables, is_connect).run();
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
            let nexts = self.edge.get(&node).unwrap_or(&EMPTY_USIZE);
            for next in nexts {
                nodes.push(*next);
            }
        }
        output
    }

    fn topo_recur(&self, visit: &mut Set<usize>,output: &mut Vec<Set<usize>>,  node_group_id: usize){
        if visit.contains(&node_group_id) {
            return;
        }
        visit.insert(node_group_id);
        for next_node in self.edge.get(&node_group_id).unwrap_or(&EMPTY_USIZE) {
            self.topo_recur(visit, output, *next_node);
        }
        output.push(self.node_group[node_group_id].clone());
    }

    pub fn toposort(&self) -> Vec<Set<Variable>> {
        let mut output = vec![];
        let mut visit = Set::new();
        for (node_group_id, _) in self.node_group.iter().enumerate() {
            self.topo_recur(&mut visit, &mut output, node_group_id)
        }
        output.iter().map(|variableset| variableset.iter().map(|var_id| self.variables[*var_id]).collect()).collect()
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
