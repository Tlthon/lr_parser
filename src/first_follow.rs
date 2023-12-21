use crate::{syntax::{Rule, Terminal, Variable}};

use std::collections::{HashSet as Set};

use std::iter;
use once_cell::sync::Lazy;
use crate::data_structure::map_set::MapSet;
use crate::data_structure::JoinAble;

static DEFAULT_MAPSET:Lazy<MapSet<Variable, Terminal>> = Lazy::new(||MapSet::default());

fn track_adding<T, Key, Value>(current_val: &Key, track: &MapSet<Key, Key>, visited: &mut Set<Key>, map: &mut MapSet<Key, Value, T>)
where T: JoinAble + Clone,
      Key: Copy + std::hash::Hash + std::cmp::Eq,
      Value: std::hash::Hash+ Eq + Clone{
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
    map: MapSet<Variable, Terminal, MapSet<usize, Terminal>>,
    rule_track: MapSet<Variable, usize>,
}

impl Follow {
    pub fn new(first_set: &First, rules: &[Rule]) -> Self{
        let mut track: MapSet<Variable, Variable> = MapSet::default();
        let mut rule_track: MapSet<Variable, usize> = MapSet::default();

        let mut map: MapSet<Variable, Terminal, MapSet<usize, Terminal>> = MapSet::default();
        map.add(Variable::accept(), (0, iter::once(Terminal::end()).collect()));
        for (rule_id, rule) in rules.iter().enumerate(){
            let clause = rule.clause;
            for (id, variable) in rule.output.data.iter().enumerate().filter_map(|(id, char)| Some((id, char.try_into().ok()?))) {
                let Some(next) = rule.output.data.get(id + 1) else {
                    
                    // A-> pB is a production
                    if variable != clause {
                        rule_track.add(variable, rule_id);
                        track.add(variable, clause);
                    }
                    break;
                };
                // A-> pBq is a production
                use crate::syntax::MixedChar::{Terminal, Variable};
                match next {
                    Terminal(next_ter) => map.add_once(variable, rule_id, next_ter.to_owned()),
                    Variable(next_var) => {
                        // if q -> Ɛ is a production act as if A->pB is also production
                        if first_set.empty.contains(next_var) && variable != clause{
                            rule_track.add(variable, rule_id);
                            track.add(variable, clause);
                        }
                        // map.append(variable, first_set.map.get(next_var).to_owned())
                        map.append(variable, MapSet::new(rule_id, first_set.map.get(next_var).to_owned()))
                        // follows.append(first_set.map.get(next_var).into());
                    },
                }
            }
        }
        for variable in track.keys() {
            track_adding(variable, &track,&mut Set::new(), &mut map);
            track_adding(variable, &track,&mut Set::new(), &mut rule_track);
        }

        Self{map, rule_track}
    }
    pub fn get(&self, key: &Variable) -> Set<Terminal> {
        self.map.get(key).all().unwrap_or_default()
    }
    pub fn get_filtered<'a>(&self, key: &Variable,
                            available_rule: impl IntoIterator<Item = &'a usize>,
                            kernel_follow: impl IntoIterator<Item = (usize, Terminal)>) -> Set<Terminal> {
        let mut first_val = self.map.get(key).aggregate(available_rule).unwrap_or_default(); // This is from A->aBc
        let rules = self.rule_track.get(key);
        for (rule_id, rule_follow) in kernel_follow{
            if rules.contains(&rule_id) {
                first_val.insert(rule_follow);
            }
        }
        first_val
    }

    pub fn print(&self) {
        println!("Follow set");
        for (var, terminals) in self.map.iter() {
            println!("{} {:?}", var, terminals);
        }
    }
}