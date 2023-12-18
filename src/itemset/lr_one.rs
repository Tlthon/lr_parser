use std::collections::{HashMap};
use std::iter;
use crate::rule_depend::RuleGraph;
use crate::{first_follow, syntax};
use crate::itemset::item_lookahead::ItemSet;
use crate::syntax::{MixedChar, Rule};

// mod display;

pub struct ItemSets {
    pub sets: Vec<ItemSet>,
    pub rules: Vec<Rule>,
    pub ordering_map: Vec<Vec<(MixedChar, usize)>>,
}

impl super::ItemSets<'_> for ItemSets {
    type Item = super::item_lookahead::Item;
    type ItemSet = ItemSet;
    fn item_sets(&self) -> &[Self::ItemSet] { &self.sets }
    fn rules(&self) -> &[Rule] { &self.rules }
}

impl ItemSets {
    pub fn new(last_variable: char) -> Self {
        Self { rules: vec![Rule::end(last_variable)], sets: Vec::default(), ordering_map: Vec::default() }
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
        first_item.add_kernel(&self.rules[0], 0, 0, &[syntax::Terminal::epsilon()]);
        let first = first_follow::First::from_rule(&self.rules);
        let follows = first_follow::Follow::new(&first, &self.rules);
        let closures: Vec<usize> = rule_graph.gets_rule(first_item.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));

        for closure in &closures {
            let output = &self.rules[*closure].clause;
            let follow_set = follows.get_filtered(output, iter::once(&index).chain(&closures), []);
            first_item.add_rule(&self.rules[*closure], 0, *closure, &follow_set);
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
                    let non_kernels: Vec<usize> = rule_graph.gets_rule(new_itemset.symbols.iter().filter_map(|symbol| symbol.try_into().ok()));
                    for non_kernel in  non_kernels.iter(){
                        let kernels = new_itemset.items.iter().map(|item| &item.rule_number);
                        let output = &self.rules[*non_kernel].clause;
                        let follow_set = follows.get_filtered(output, kernels.chain(&non_kernels), cur_item.kernel_follow());
                        new_itemset.add_rule(&self.rules[*non_kernel], 0, *non_kernel, &follow_set)
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
