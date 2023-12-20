use std::collections::BTreeSet;
use std::iter;
use crate::first_follow::Follow;
use crate::itemset::Item as _;
use crate::rule_depend::RuleGraph;
use crate::syntax::{MixedChar, Rule, Terminal};

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Item{
    pub(crate) rule_number: usize,
    kernel: bool,
    dot: usize,
    follow: Terminal
}

impl<'display> super::Item<'display> for Item {
    type Display = super::display::item_lookahead::ItemDisplay<'display>;

    fn shift(&self) -> Self {
        Self { rule_number: self.rule_number, dot: self.dot + 1, kernel: true , follow: self.follow}
    }
    fn symbol(&self, rules: &[Rule]) -> Option<MixedChar> {
        rules[self.rule_number].output.data.get(self.dot).copied()
    }

    fn is_end(&self, rules: &[Rule]) -> bool {
        rules[self.rule_number].output.data.len() == self.dot
    }

    fn display(&'display self, rules: &'display [Rule]) -> Self::Display{
        super::display::item_lookahead::ItemDisplay{
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
    pub fn new(rule_number: usize, dot:usize, follow: Terminal, kernel: bool) -> Self {
        Self { rule_number, dot, kernel, follow}
    }

    pub fn follow(&self) -> Terminal {self.follow}
}

#[derive(Hash,Eq,PartialEq,Clone, Debug)]
pub struct ItemSet{
    pub items: BTreeSet<Item>,
    pub(crate) symbols: BTreeSet<MixedChar>,
}

impl<'item> super::ItemSet<'item> for ItemSet {
    type Item = Item;
    type ItemIterator = std::collections::btree_set::Iter<'item, Self::Item>;
    fn items(&'item self) -> Self::ItemIterator { self.items.iter() }
}

impl ItemSet {
    pub(crate) fn new() -> Self {
        Self { items: BTreeSet::default(), symbols: BTreeSet::default()}
    }
    pub(crate) fn add_rule<'a, Terminals>(&mut self, rule:&Rule, dot:usize, rule_number:usize, follows: Terminals)
        where Terminals: IntoIterator<Item = &'a Terminal>
    {
        for follow in follows {
            self.items.insert(Item::new(rule_number, dot ,*follow, false));
        }

        if let Some(character) = rule.output.data.get(dot){
            self.symbols.insert(*character);
        }
    }
    pub(crate) fn add_kernel<'a, Terminals>(&mut self, rule:&Rule, dot:usize, rule_number:usize, follows: Terminals)
        where Terminals: IntoIterator<Item = &'a Terminal>
    {
        for follow in follows {
            self.items.insert(Item::new(rule_number, dot ,*follow, true));
        }

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

    pub fn reduce<'a>(&'a self, rules:&'a [Rule]) -> impl Iterator<Item = (Rule, Terminal)> + 'a {
        self.items.iter().filter_map(|item| {
            match item.is_end(rules) {
                true => Some((rules[item.rule_number].clone(), item.follow)),
                false => None,
            }
        })
    }

    pub fn kernel_follow<'a>(&'a self, rules: &'a [Rule]) -> impl Iterator<Item = (usize, Terminal)> + 'a{
        self.items.iter().filter(|item| item.kernel)
            .filter(|item| item.is_end(rules))
            .map(|kernel| (kernel.rule_number, kernel.follow))
    }

    fn empty_follow<'a>(&'a self) -> impl Iterator<Item = (usize, Terminal)> + 'a {
        iter::empty()
    }

    pub(super) fn merge(&mut self, other: Self) {
        for item in other.items {
            self.items.insert(item);
        }
        for symbol in other.symbols {
            self.symbols.insert(symbol);
        }
    }

    pub(super) fn add_non_kernel(&mut self, rule_graph: &RuleGraph, rules: &[Rule], follows: &Follow, prev: Option<&ItemSet>) {
        let non_kernels: Vec<usize> = rule_graph.gets_rule(self.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));
        for non_kernel in non_kernels.iter() {
            let kernels = self.items.iter().map(|item| &item.rule_number);
            let output = rules[*non_kernel].clause;
            let Some(prev_set) = prev else {
                let follow_set = follows.get_filtered(&output, kernels.chain(&non_kernels), iter::empty());
                self.add_rule(&rules[*non_kernel], 0, *non_kernel, &follow_set);
                continue
            };
            let follow_set = follows.get_filtered(&output, kernels.chain(&non_kernels), prev_set.kernel_follow(rules));
            self.add_rule(&rules[*non_kernel], 0, *non_kernel, &follow_set)
        }
    }

    pub(super) fn get_non_kernel(&self, rule_graph: &RuleGraph, rules: &[Rule], follows: &Follow, prev: Option<&ItemSet>) -> Self{
        let mut next = self.clone();
        next.add_non_kernel(rule_graph, rules, follows, prev);
        next
    }
}