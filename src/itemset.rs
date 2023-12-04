use std::{collections::{BTreeSet, HashMap}, fmt::Display};
pub const DOT: char = '•';

use crate::{syntax::{Rule, MixedChar}, ruledepend::RuleGraph, firstfollow};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Item{
    rule_number: usize,
    dot: usize
}

impl Item {
    pub fn new(rule_number: usize, dot:usize) -> Self {
        Self { rule_number, dot}
    }

    pub fn shift(&self) -> Self {
        Self { rule_number: self.rule_number, dot: self.dot + 1 }
    }
    pub fn symbol(&self, rules: &[Rule]) -> Option<MixedChar> {
        rules[self.rule_number].output.data.get(self.dot).copied()
    }

    fn is_end(&self, rules: &[Rule]) -> bool {
        rules[self.rule_number].output.data.len() == self.dot
    }
}

#[derive(Hash,Eq,PartialEq,Clone, Debug)]
pub struct ItemSet{
    items: Vec<Item>,
    symbols: BTreeSet<MixedChar>,
}

impl ItemSet {
    fn new() -> Self {
        Self { items: vec![], symbols: BTreeSet::default()}
    }
    fn add_rule(&mut self, rule:&Rule, dot:usize, rule_number:usize) {
        self.items.push(Item::new(rule_number, dot));

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

    pub fn reduce<'a>(&'a self, rules:&'a [Rule]) -> impl Iterator<Item = Rule> + 'a {
        self.items.iter().filter_map(|item| {
            match item.is_end(rules) {
                true => Some(rules[item.rule_number].clone()),
                false => None,
            }
        })
    } 
}
pub struct ItemSets {
    pub itemsets: Vec<ItemSet>,
    pub rules: Vec<Rule>,
    pub ordering_map: Vec<Vec<(MixedChar, usize)>>,
}

impl ItemSets {
    pub fn new() -> Self {
        Self { rules: Vec::default(), itemsets: Vec::default(), ordering_map: Vec::default() }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_from_string(&mut self, string_rule: &str) -> Option<()>{
        self.rules.push(string_rule.try_into().ok()?);
        Some(())
    }

    pub fn generate_next(&mut self){
        let mut itemmaps = HashMap::new();
        let rulegraph = RuleGraph::new(self.rules.clone());
        let mut first_item = ItemSet::new();
        let mut index = 0;

        first_item.add_rule(&self.rules[0], 0, 0);
        let first = firstfollow::First::from_rule(&self.rules);
        first.print();
        let follow = firstfollow::Follow::new(&first, &self.rules);
        follow.print();
        println!("{:?}", rulegraph.toposort());
        let kernellist: Vec<usize> = rulegraph.gets_rule(first_item.symbols.iter().filter_map(|symbol| symbol.try_variable()));
        for kernel in kernellist {
            first_item.add_rule(&self.rules[kernel], 0, kernel)
        }
        self.itemsets.push(first_item);
        loop {
            let Some(cur_item) = self.itemsets.get(index).cloned() else {
                break;
            };
            let symbols = & mut cur_item.symbols.clone();

            let mut next_val = Vec::new();
            for transchar in symbols.iter(){
                let new_itemset: Option<ItemSet> = cur_item.transitions(*transchar, &self.rules);
                if let Some(mut new_itemset) = new_itemset {
                    let kernellist: Vec<usize> = rulegraph.gets_rule(new_itemset.symbols.iter().filter_map(|symbol| symbol.try_variable()));
                    for kernel in kernellist {
                        new_itemset.add_rule(&self.rules[kernel], 0, kernel)
                    }
                    if let Some(new_index) = itemmaps.get(&new_itemset) {
                        next_val.push((transchar.clone(), *new_index));
                        continue;
                    } 
                    itemmaps.insert(new_itemset.clone(), self.itemsets.len());
                    next_val.push((transchar.clone(), self.itemsets.len()));
                    self.itemsets.push(new_itemset);
                }
            }
            self.ordering_map.push(next_val);
            index += 1;
        }

    }
}

impl Display for ItemSets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (number, item_set) in self.itemsets.iter().enumerate() {
            write!(f, "Item set {}\n",number)?;
            for item in &item_set.items {
                write!(f, "{} -> ", self.rules[item.rule_number].clause)?;
                if 0 == item.dot {
                    write!(f, "{} ",DOT)?;
                }
                for (i, character) in self.rules[item.rule_number].output.data.iter().enumerate() {
                    write!(f, "{} ",character)?;
                    if i + 1 == item.dot {
                        write!(f, "{} ",DOT)?;
                    }    
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}