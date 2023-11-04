use std::{char, fmt::Display};

pub const END_TERMINAL: char =  '\u{FDD0}';
pub const END_VARIABLE: char =  '\u{FDD1}';

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Terminal{
    pub symbol: char
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_TERMINAL {
            return write!(f, "eof");
            
        }
        write!(f, "{}", self.symbol)
    }
}

impl From<&char> for Terminal {
    fn from(value: &char) -> Self {
        Self { symbol: *value }
    }
}


#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Variable{
    pub symbol: char
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_VARIABLE {
            return write!(f, "acc");
            
        }
        write!(f, "{}", self.symbol)
    }
}


#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum MixedChar{
    Terminal(Terminal),
    Variable(Variable)
}

impl MixedChar {
    pub fn try_variable(self) -> Option<Variable> {
        match self {
            MixedChar::Terminal(_) => None,
            MixedChar::Variable(v) => Some(v),
        }
    }
}
impl From<Variable> for MixedChar {
    fn from(value: Variable) -> Self {
        MixedChar::Variable(value)
    }
}

impl Display for MixedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedChar::Terminal(t) => write!(f, "{}", t),
            MixedChar::Variable(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone)]
pub struct MixedString{
    pub data: Vec<MixedChar>
}

pub struct TerminalString{
    pub data: Vec<Terminal>
}
#[derive(Clone)]
pub struct Rule{
    pub clause: Variable,
    pub output: MixedString
}

impl Rule {
    pub fn new(start: char) -> Self {
        Self { 
            clause: Variable{symbol: start }, 
            output: MixedString{data: Vec::new()}
        }
    }
    pub fn add_terminal(&mut self,terminal: char){
        self.output.data.push(MixedChar::Terminal(Terminal { symbol: terminal }))
    }

    pub fn add_variable(&mut self,variable: char){
        self.output.data.push(MixedChar::Variable(Variable { symbol: variable }))
    }

    pub fn len(&self) -> usize {
        self.output.data.len()
    }
}

pub struct Rules{
    pub rules: Vec<Rule>
}