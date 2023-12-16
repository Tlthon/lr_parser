use std::fmt::{Debug, Display};
use crate::syntax::{END_TERMINAL, END_VARIABLE, MixedChar, MixedString, Rule, Terminal, TerminalString, Variable};

macro_rules! write_pad {
    ($dst:expr, $($arg:tt)*) => {
        $dst.pad(&format!($($arg)*))
    };
}


impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_TERMINAL {
            // return write!(f, "EOF");
            return f.pad(&format!("EOF"))
        }
        return f.pad(&format!("{:.1}", self.symbol))
    }
}
impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_VARIABLE {
            return write_pad!(f, "{:.6}" ,"ACCEPT");

        }
        write_pad!(f, "{:.1}", self.symbol)
    }
}
impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}
impl Display for MixedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedChar::Terminal(t) => write_pad!(f, "{:.6}", t),
            MixedChar::Variable(v) => write_pad!(f, "{:.3}", v),
        }
    }
}
impl Debug for MixedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl MixedChar {
    pub fn display_len(&self) -> usize{
        match self {
            MixedChar::Terminal(t) if t.symbol == END_TERMINAL => 3,
            MixedChar::Variable(v) if v.symbol == END_VARIABLE => 6,
            _ => 1

        }
    }
}

impl Display for MixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for character in self.data.iter() {
            write!(f, "{} ", character)?
        }
        if self.data.len() == 0 {
            write!(f, "\u{03B5} ")?;
        }

        return Ok(());
    }
}
impl Debug for MixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for character in self.data.iter() {
            if first != true {
                write!(f, ", ")?
            }
            first = false;
            write!(f, "'{}'", character)?
        }
        write!(f, "]")?;
        return Ok(());
    }
}
impl Display for TerminalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for character in self.data.iter() {
            write!(f, "{}", character)?
        }
        return Ok(());
    }
}
impl Debug for TerminalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for character in self.data.iter() {
            if first != true {
                write!(f, ", ")?
            }
            first = false;
            write!(f, "'{}'", character)?
        }
        write!(f, "]")?;
        return Ok(());
    }
}
impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->", self.clause)?;
        write!(f, "{}", self.output)?;
        Ok(())
    }
}

