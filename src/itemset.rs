use std::{collections::BTreeSet, fmt::Display, usize};
pub const DOT: char = '•';

use crate::{syntax::{Rule, MixedChar, Variable}, ruledepend::RuleGraph};

#[derive(Hash, PartialEq, Eq, Clone)]
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

#[derive(Hash,Eq,PartialEq,Clone)]
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
            // let rule = &rules[*rule_number];
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

    pub fn reduce(&self, rules:&[Rule]) -> Option<(usize, Variable)> {
        for item in &self.items {
            if item.is_end(rules) {
                return Some((item.dot, rules[item.rule_number].clause));
            }
        }
        None
    } 
}
pub struct ItemSets {
    pub itemset: Vec<ItemSet>,
    pub rules: Vec<Rule>
}

impl ItemSets {
    pub fn new() -> Self {
        Self { itemset: vec![ItemSet::new()], rules: Vec::default() }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.itemset[0].add_rule(&rule, 0, self.rules.len());
        self.rules.push(rule);
    }

    pub fn generate_next(&mut self){
        let mut i = 0;
        let rulegraph = RuleGraph::new(self.rules.clone());
        while i < self.itemset.len() {
            let symbols = & mut self.itemset[i].symbols.clone();
            for transchar in symbols.iter(){
                let new_itemset = self.itemset[i].transitions(*transchar, &self.rules);
                if let Some(new_itemset) = new_itemset {
                    self.itemset.push(new_itemset);
                }
            }
            if i != 0 {
                let kernellist: Vec<usize> = rulegraph.gets_rule(symbols.iter().filter_map(|symbol| symbol.try_variable()));
                for kernel in kernellist {
                    self.itemset[i].add_rule(&self.rules[kernel], 0, kernel)
                }
            }
            i = i+1;

        }
    }
}

impl Display for ItemSets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (number, item_set) in self.itemset.iter().enumerate() {
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