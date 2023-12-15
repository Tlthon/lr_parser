use std::{collections::{BTreeSet, HashMap}, fmt::Display};
use crate::{syntax::{Rule, MixedChar}, rule_depend::RuleGraph, first_follow, syntax};
use crate::syntax::Terminal;
pub const DOT: char = '•';
mod display;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Item{
    rule_number: usize,
    kernel: bool,
    dot: usize,
    follow: Terminal
}

impl Item {
    pub fn new(rule_number: usize, dot:usize, follow: Terminal) -> Self {
        Self { rule_number, dot, kernel: false, follow}
    }

    pub fn shift(&self) -> Self {
        Self { rule_number: self.rule_number, dot: self.dot + 1, kernel: true , follow: self.follow}
    }
    pub fn symbol(&self, rules: &[Rule]) -> Option<MixedChar> {
        rules[self.rule_number].output.data.get(self.dot).copied()
    }

    fn is_end(&self, rules: &[Rule]) -> bool {
        rules[self.rule_number].output.data.len() == self.dot
    }

    pub fn display<'a> (&'a self, rules: &'a [Rule]) -> display::ItemDisplay<'a> {
        display::ItemDisplay{
            item: self,
            rules
        }
    }

    pub fn kernel(&self) -> bool {
        self.kernel
    }
}

#[derive(Hash,Eq,PartialEq,Clone, Debug)]
pub struct ItemSet{
    pub items: Vec<Item>,
    symbols: BTreeSet<MixedChar>,
}

impl ItemSet {
    fn new() -> Self {
        Self { items: vec![], symbols: BTreeSet::default()}
    }
    fn add_rule<'a, Terminals>(&mut self, rule:&Rule, dot:usize, rule_number:usize, follows: Terminals)
    where Terminals: IntoIterator<Item = &'a Terminal>
    {

        for follow in follows {
            self.items.push(Item::new(rule_number, dot, *follow));
        }

        if let Some(character) = rule.output.data.get(dot){
            self.symbols.insert(*character);
        }
    }

    fn add_item(&mut self, rules: &[Rule], item: Item) {
        self.items.push(item);

        if let Some(character) = self.items.last().and_then(|x| x.symbol(rules)){
            self.symbols.insert(character);
        }
    }

    fn transitions(&self, transchar: MixedChar, rules: &[Rule]) -> Option<ItemSet> {
        let mut new_set = Self::new();
        for item in &self.items {
            if item.symbol(rules) == Some(transchar) {
                new_set.add_item(rules, item.shift());
            }
        }
        if new_set.items.len() == 0 {
            return None;
        }
        Some(new_set)
    }

    pub fn is_previous(&self, rules: &[Rule], other: &Self) -> Option<MixedChar> {
        for item in &self.items {
            for other_item in &other.items {
                if item.dot + 1 == other_item.dot && item.rule_number == other_item.rule_number {
                    return Some(item.symbol(rules).unwrap());
                }
            }
        }
        return None;
    }

    pub fn reduce<'a>(&'a self, rules:&'a [Rule]) -> impl Iterator<Item = (Rule, Terminal)> + 'a {
        self.items.iter().filter_map(|item| {
            match item.is_end(rules) {
                true => Some((rules[item.rule_number].clone(), item.follow)),
                false => None,
            }
        })
    }
}
pub struct ItemSets {
    pub sets: Vec<ItemSet>,
    pub rules: Vec<Rule>,
    pub ordering_map: Vec<Vec<(MixedChar, usize)>>,
}

impl ItemSets {
    pub fn new() -> Self {
        Self { rules: Vec::default(), sets: Vec::default(), ordering_map: Vec::default() }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_from_string(&mut self, string_rule: &str) -> bool{
        let Ok(rule) = string_rule.try_into() else {
            return false
        };
        self.rules.push(rule);
        true
    }

    pub fn clear(&mut self) {
        self.sets = Vec::default();
        self.ordering_map = Vec::default();
    }

    pub fn generate_next(&mut self){
        let mut itemmaps = HashMap::new();
        let rule_graph = RuleGraph::new(self.rules.clone());
        let mut first_item = ItemSet::new();
        let mut index = 0;

        first_item.add_rule(&self.rules[0], 0, 0, &[syntax::Terminal::epsilon()]);
        let first = first_follow::First::from_rule(&self.rules);
        let follows = first_follow::Follow::new(&first, &self.rules);
        let kernels: Vec<usize> = rule_graph.gets_rule(first_item.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));

        for kernel in kernels {
            let output = &self.rules[kernel].clause;
            first_item.add_rule(&self.rules[kernel], 0, kernel, follows.get(output));
            first_item.items[0].kernel = true;
        }
        self.sets.push(first_item);
        loop {
            let Some(cur_item) = self.sets.get(index).cloned() else {
                break;
            };
            let symbols = & mut cur_item.symbols.clone();
            let mut next_val = Vec::new();
            for transition_char in symbols.iter(){
                let new_itemset: Option<ItemSet> = cur_item.transitions(*transition_char, &self.rules);
                if let Some(mut new_itemset) = new_itemset {
                    let kernels: Vec<usize> = rule_graph.gets_rule(new_itemset.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));
                    for kernel in  kernels{
                        new_itemset.add_rule(&self.rules[kernel], 0, kernel, follows.get(&self.rules[kernel].clause))
                    }
                    if let Some(new_index) = itemmaps.get(&new_itemset) {
                        next_val.push((transition_char.clone(), *new_index));
                        continue;
                    } 
                    itemmaps.insert(new_itemset.clone(), self.sets.len());
                    next_val.push((transition_char.clone(), self.sets.len()));
                    self.sets.push(new_itemset);
                }
            }
            self.ordering_map.push(next_val);
            index += 1;
        }
    }
}