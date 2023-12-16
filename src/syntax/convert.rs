use crate::syntax::{MixedChar, Rule, Terminal, TerminalString, Variable};
impl From<&char> for Terminal {
    fn from(value: &char) -> Self {
        Self { symbol: *value }
    }
}
impl TryFrom<&MixedChar> for Terminal {
    type Error = ();
    fn try_from(value: &MixedChar) -> Result<Self, Self::Error> {
        match value {
            MixedChar::Terminal(t) => Ok(*t),
            MixedChar::Variable(_) => Err(()),
        }
    }
}
impl TryFrom<&MixedChar> for Variable {
    type Error = ();
    fn try_from(value: &MixedChar) -> Result<Self, Self::Error> {
        match value {
            MixedChar::Terminal(_) => Err(()),
            MixedChar::Variable(v) => Ok(*v),
        }
    }
}


impl From<Variable> for MixedChar {
    fn from(value: Variable) -> Self {
        MixedChar::Variable(value)
    }
}
impl From<Terminal> for MixedChar {
    fn from(value: Terminal) -> Self {
        MixedChar::Terminal(value)
    }
}
impl From<&[char]> for TerminalString {
    fn from(value: &[char]) -> Self {
        Self{
            data: value.iter().map(|symbol| Terminal{ symbol: *symbol }).collect()
        }
    }
}
impl TryFrom<&str> for Rule {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut list = value.chars();
        let Some(first) = list.next() else {
            return Err(());
        };
        if !first.is_ascii_uppercase() {
            return Err(());
        }
        let mut rule = Rule::new(first);
        if let Some(separator) = list.next() {
            if separator != ':' {
                return Err(());
            }
        }
        while let Some(output) = list.next() {
            if output.is_ascii_uppercase() {
                rule.add_variable(output)
            }
            else {
                rule.add_terminal(output)
            }
        }
        Ok(rule)
    }
}