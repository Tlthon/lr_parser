use std::{collections::{HashMap, HashSet}, ops::BitOr};

use once_cell::sync::Lazy;

use crate::{syntax::{Variable, Terminal, Rule}, ruledepend::RuleGraph};

#[derive(Default)]
pub struct First{
    map: HashMap<Variable, HashSet<Terminal>>
}

static EMPTY_SET: Lazy<HashSet<Terminal>> = Lazy::new(|| {HashSet::new()}); 

impl First {
    fn add(&mut self, variable: Variable, terminal: Terminal) {
        self.map.entry(variable).or_default().insert(terminal);
    }
    fn join(&mut self, output: Variable, source: Variable) {
        let first = self.map.get(&source).unwrap_or(&EMPTY_SET);
        let second = self.map.get(&output).unwrap_or(&EMPTY_SET);
        self.map.insert(output, first.bitor(second));
    }
    fn set(&mut self, variable: Variable, set: HashSet<Terminal>) {
        self.map.insert(variable, set);
    }

    pub fn from_rule(rule:&[Rule], rule_graph: &RuleGraph) -> Self {
        let mut first_set = Self::default();
        let rule_tuple = rule.iter()
            .filter_map(|rule| {
                Some((rule.clause, rule.output.data.first()?.try_terminal()?))
            });
        for (variable, terminal) in rule_tuple {
            first_set.add(variable, terminal);
        }

        for mut varset in rule_graph.toposort(){
            let Some(firstvar) = varset.pop_first() else {
                continue;
            };
            for depend_var in rule_graph.get(firstvar) {
                first_set.join(firstvar, depend_var)
            }
            let true_terminal = first_set.map.get(&firstvar).unwrap().clone();
            for var in varset {
                first_set.set(var, true_terminal.clone());
            }
        }
        first_set
    }

    pub fn print(&self) {
        for (var, terminals) in &self.map {
            println!("{} {:?}", var, terminals);
        }
    }
}

#[derive(Default)]
pub struct Follow{
    map: HashMap<Variable, HashSet<Terminal>>
}

impl Follow {
    fn add(&mut self, variable: Variable, terminal: Terminal) {
        self.map.entry(variable).or_default().insert(terminal);
    }

    pub fn new(first_set: &First, rules: &[Rule]) {

    }
}