use std::collections::{HashMap};
use crate::itemset::item_no_lookahead::ItemSet;
use crate::itemset::ItemSets as _;
use crate::rule_depend::RuleGraph;

use crate::syntax::{MixedChar, Rule};
// mod display;

pub struct ItemSets {
    pub sets: Vec<ItemSet>,
    pub rules: Vec<Rule>,
    pub ordering_map: Vec<Vec<(MixedChar, usize)>>,
}

impl super::ItemSets<'_> for ItemSets {
    type Item = super::item_no_lookahead::Item;
    type ItemSet = super::item_no_lookahead::ItemSet;
    fn item_sets(&self) -> &[Self::ItemSet] { &self.sets }
    fn rules(&self) -> &[Rule] {
        &self.rules
    }
    fn ordering_map(&self) -> &[Vec<(MixedChar, usize)>] {
        self.ordering_map.as_slice()
    }

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
        first_item.add_kernel(&self.rules[0], 0, 0);
        first_item.add_non_kernel(&rule_graph, &self.rules);
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
                    new_itemset.add_non_kernel(&rule_graph, &self.rules);
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
