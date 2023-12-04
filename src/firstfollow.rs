use std::collections::{HashMap, HashSet};

use crate::{syntax::{Variable, Terminal, Rule}, ruledepend::RuleGraph, mapset::MapSet};

fn track_adding(current_val: &Variable, track: &MapSet<Variable, Variable>, visited: &mut HashSet<Variable>, map: &mut MapSet<Variable, Terminal>) {
    if visited.contains(current_val) {
        return;
    }
    visited.insert(*current_val);
    for next_variable in track.get(current_val) {
        track_adding(next_variable, track, visited, map);
        map.join(*current_val,* next_variable);
    }
    
}


#[derive(Default)]
pub struct First{
    map: MapSet<Variable, Terminal>
}
impl First {
    pub fn from_rule(rule:&[Rule], rule_graph: &RuleGraph) -> Self {
        let mut first_set = Self::default();
        let rule_tuple = rule.iter()
            .filter_map(|rule| {
                Some((rule.clause, rule.output.data.first()?.try_terminal()?))
            });
        for (variable, terminal) in rule_tuple {
            first_set.map.add(variable, terminal);
        }

        for mut varset in rule_graph.toposort(){
            let Some(firstvar) = varset.pop_first() else {
                continue;
            };
            for depend_var in rule_graph.get(firstvar) {
                first_set.map.join(firstvar, depend_var)
            }
            let true_terminal = first_set.map.get(&firstvar).clone();
            for var in varset {
                first_set.map.set(var, true_terminal.clone());
            }
        }
        first_set
    }

    pub fn print(&self) {
        println!("First set");

        for (var, terminals) in self.map.iter() {
            println!("{} {:?}", var, terminals);
        }
    }
}

#[derive(Default)]
pub struct Follow{
    map: MapSet<Variable, Terminal>
}

impl Follow {
    pub fn new(first_set: &First, rules: &[Rule]) -> Self{
        let mut track: MapSet<Variable, Variable>= MapSet::default();
        let mut map: MapSet<Variable, Terminal> = MapSet::default();
        for rule in rules{
            let clause = rule.clause;
            for (id, variable) in rule.output.data.iter().enumerate().filter_map(|(id, char)| Some((id, char.try_variable()?))) {
                let Some(next) = rule.output.data.get(id + 1) else {
                    
                    // A-> pB is a production
                    if variable != clause {
                        track.add(variable, clause);
                    }
                    break;
                };
                // A-> pBq is a production
                match next {
                    crate::syntax::MixedChar::Terminal(next_ter) => map.add(variable, *next_ter),
                    crate::syntax::MixedChar::Variable(next_var) => map.append(variable, first_set.map.get(next_var).clone()),
                }
            }
        }
        println!("track: {:?}", track);

        for variable in track.keys() {
            track_adding(variable, &track,&mut HashSet::new(), &mut map)
        }

        Self{map}
    }
    pub fn get(&self, key: &Variable) -> &HashSet<Terminal> {
        self.map.get(key)
    }
    pub fn print(&self) {
        println!("Follow set");
        for (var, terminals) in self.map.iter() {
            println!("{} {:?}", var, terminals);
        }
    }
}