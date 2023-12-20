use std::collections::BTreeSet;
use crate::rule_depend::RuleGraph;
use crate::syntax::{MixedChar, Rule};
use super::Item as _;
use super::ItemSet as _;

pub const DOT: char = '•';

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Item{
    pub(crate) rule_number: usize,
    kernel: bool,
    pub(crate) dot: usize,
}

impl<'display> super::Item<'display> for Item {
    type Display = super::display::item_no_lookahead::ItemDisplay<'display>;
    fn shift(&self) -> Self {
        Self { rule_number: self.rule_number, dot: self.dot + 1, kernel: true}
    }
    fn symbol(&self, rules: &[Rule]) -> Option<MixedChar> {
        rules[self.rule_number].output.data.get(self.dot).copied()
    }

    fn is_end(&self, rules: &[Rule]) -> bool {
        rules[self.rule_number].output.data.len() == self.dot
    }

    fn display(&'display self, rules: &'display [Rule]) -> Self::Display{
        Self::Display{
            item: self,
            rules
        }
    }
    fn dot(&self) -> usize {self.dot}
    fn kernel(&self) -> bool {
        self.kernel
    }
}

impl Item {
    pub fn new(rule_number: usize, dot:usize, kernel: bool) -> Self {
        Self { rule_number, dot, kernel}
    }
}

#[derive(Hash,Eq,PartialEq,Clone, Debug)]
pub struct ItemSet{
    pub items: BTreeSet<Item>,
    pub(crate) symbols: BTreeSet<MixedChar>,
}

impl<'item_iterator> super::ItemSet<'item_iterator> for ItemSet {
    type Item = Item;
    type ItemIterator = std::collections::btree_set::Iter<'item_iterator, Item>;
    fn items(&'item_iterator self) -> Self::ItemIterator { self.items.iter() }
}

impl ItemSet {
    pub(crate) fn new() -> Self {
        Self { items: BTreeSet::default(), symbols: BTreeSet::default()}
    }
    pub(crate) fn add_rule(&mut self, rule:&Rule, dot:usize, rule_number:usize)
    {
        self.items.insert(Item::new(rule_number, dot, false));

        if let Some(character) = rule.output.data.get(dot){
            self.symbols.insert(*character);
        }
    }
    pub(crate) fn add_kernel(&mut self, rule:&Rule, dot:usize, rule_number:usize)
    {
        self.items.insert(Item::new(rule_number, dot, true));

        if let Some(character) = rule.output.data.get(dot){
            self.symbols.insert(*character);
        }
    }

    fn add_item(&mut self, rules: &[Rule], item: Item) {
        if let Some(character) = item.symbol(rules){
            self.symbols.insert(character);
        }
        self.items.insert(item);
    }

    pub(crate) fn transitions(&self, transchar: MixedChar, rules: &[Rule]) -> Option<ItemSet> {
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

    pub(super) fn add_non_kernel(&mut self, rule_graph: &RuleGraph, rules: &[Rule]) {
        let non_kernels: Vec<usize> = rule_graph.gets_rule(self.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));
        for non_kernel in  non_kernels.iter(){
            self.add_rule(&rules[*non_kernel], 0, *non_kernel);
        }
    }

    pub(super) fn get_non_kernel(&self, rule_graph: &RuleGraph, rules: &[Rule]) -> Self{
        let mut next = self.clone();
        let non_kernels: Vec<usize> = rule_graph.gets_rule(self.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));
        for non_kernel in  non_kernels.iter(){
            next.add_rule(&rules[*non_kernel], 0, *non_kernel);
        }
        next
    }

}
use super::item_lookahead::ItemSet as ItemSetLookahead;

impl From<&ItemSetLookahead> for ItemSet {
    fn from(value: &ItemSetLookahead) -> Self {
        let mut s: Self = Self::new();
        for item in value.items() {
            s.items.insert(Item{kernel: item.kernel(), rule_number: item.rule_number, dot: item.dot()});
        }
        let items = value.items()
            .map( |item| {Item{kernel: item.kernel(), rule_number: item.rule_number, dot: item.dot()}} )
            .collect();
        let symbols = value.symbols.clone();
        Self{ items, symbols }
    }
}