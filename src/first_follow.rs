use crate::{syntax::{Variable, Terminal, Rule}, map_set::MapSet};

use std::collections::HashSet as Set;

fn track_adding(current_val: &Variable, track: &MapSet<Variable, Variable>, visited: &mut Set<Variable>, map: &mut MapSet<Variable, Terminal>) {
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
    map: MapSet<Variable, Terminal>,
    empty: Set<Variable>
}
impl First {
    pub fn from_rule(rule:&[Rule]) -> Self {
        let mut map: MapSet<Variable, Terminal> = MapSet::default();
        let mut track:MapSet<Variable, Variable> = MapSet::default();
        let mut empty = Set::default();
        for (variable, first) in rule.iter().map(|rule| (rule.clause, rule.output.data.first())) {
            use crate::syntax::MixedChar::{Terminal, Variable};
            match first {
                Some(Terminal(next_ter)) => map.add(variable, *next_ter), // E -> aB
                Some(Variable(next_var)) => track.add(variable, *next_var), // E -> B
                None => { empty.insert(variable); }, // E -> Ɛ
            }
        }
        for variable in track.keys() {
            track_adding(variable, &track,&mut Set::new(), &mut map)
        }
        Self { map, empty }
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
        let mut track: MapSet<Variable, Variable> = MapSet::default();
        let mut map: MapSet<Variable, Terminal> = MapSet::default();
        map.add(Variable::accept(), Terminal::epsilon());
        for rule in rules{
            let clause = rule.clause;
            for (id, variable) in rule.output.data.iter().enumerate().filter_map(|(id, char)| Some((id, char.try_into().ok()?))) {
                let Some(next) = rule.output.data.get(id + 1) else {
                    
                    // A-> pB is a production
                    if variable != clause {
                        track.add(variable, clause);
                    }
                    break;
                };
                // A-> pBq is a production
                use crate::syntax::MixedChar::{Terminal, Variable};
                match next {
                    Terminal(next_ter) => map.add(variable, next_ter.to_owned()),
                    Variable(next_var) => {
                        // if q -> Ɛ is a production act as if A->pB is also production
                        if first_set.empty.contains(next_var) && variable != clause{
                            track.add(variable, clause);
                        }
                        map.append(variable, first_set.map.get(next_var).to_owned())
                    },
                }
            }
        }
        for variable in track.keys() {
            track_adding(variable, &track,&mut Set::new(), &mut map)
        }

        Self{map}
    }
    pub fn get(&self, key: &Variable) -> &Set<Terminal> {
        self.map.get(key)
    }
    pub fn print(&self) {
        println!("Follow set");
        for (var, terminals) in self.map.iter() {
            println!("{} {:?}", var, terminals);
        }
    }
}