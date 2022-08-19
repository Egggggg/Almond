// use core::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub enum BaseOutput {
    Str(String),
    Bool(bool),
    Int(i32),
    Float(f64),
    Path(RefPath),
}

pub trait Output {
    type Group;

    fn len(&self) -> usize {
        0
    }
}

#[derive(Clone, Debug)]
pub struct RefPath {
    pub parts: Vec<String>,
}

impl Output for BaseOutput {
    type Group = BaseOutput;

    fn len(&self) -> usize {
        match self {
            BaseOutput::Str(e) => e.as_bytes().len(),
            BaseOutput::Bool(e) => e.to_string().as_bytes().len(),
            BaseOutput::Int(e) => e.to_string().as_bytes().len(),
            BaseOutput::Float(e) => e.to_string().as_bytes().len(),
            BaseOutput::Path(e) => e.to_string().as_bytes().len(),
        }
    }
}

impl<T: Output> Output for Vec<T> {
    type Group = Vec<T>;

    fn len(&self) -> usize {
        self.len()
    }
}

impl fmt::Display for RefPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = "".to_owned();

        for i in self.parts.iter() {
            out.push('/');
            out.push_str(i.as_str())
        }

        write!(f, "{}", out)
    }
}

/*
impl Ord for OutputType {
    fn cmp(&self, other: &OutputType) -> Ordering {
        match (self, other) {
            (OutputType::Literal(e), OutputType::Literal(s)) => e.cmp(s),
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for OutputType {
    fn partial_cmp(&self, other: &OutputType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<String> for OutputType {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        if let OutputType::Literal(e) = self {
            return e.partial_cmp(other);
        }

        Some(Ordering::Less)
    }
}

impl PartialOrd<OutputType> for String {
    fn partial_cmp(&self, other: &OutputType) -> Option<Ordering> {
        Some(other.partial_cmp(self)?.reverse())
    }
}

impl Eq for OutputType {}

impl PartialEq for OutputType {
    fn eq(&self, other: &OutputType) -> bool {
        match (self, other) {
            (OutputType::Literal(e), OutputType::Literal(s)) => e == s,
            _ => false,
        }
    }
}

impl PartialEq<String> for OutputType {
    fn eq(&self, other: &String) -> bool {
        if let OutputType::Literal(e) = self {
            return e == other;
        }

        false
    }
}

impl PartialEq<OutputType> for String {
    fn eq(&self, other: &OutputType) -> bool {
        other == self
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Literal) -> Ordering {
        match (self, other) {
            (Literal::Str(e), Literal::Str(s)) => e.cmp(s),
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<String> for Literal {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.cmp(&Literal::Str(other.to_owned())))
    }
}

impl Eq for Literal {}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::Str(e), Literal::Str(s)) => e == s,
            _ => false,
        }
    }
}

impl PartialEq<String> for Literal {
    fn eq(&self, other: &String) -> bool {
        self == &Literal::Str(other.to_owned())
    }
}

impl From<String> for OutputType {
    fn from(old: String) -> OutputType {
        OutputType::Literal(Literal::Str(old))
    }
}

impl From<i64> for OutputType {
    fn from(old: i64) -> OutputType {
        OutputType::Literal(Literal::Int(old))
    }
}

impl From<f64> for OutputType {
    fn from(old: f64) -> OutputType {
        OutputType::Literal(Literal::Float(old))
    }
}

impl From<Vec<Literal>> for OutputType {
    fn from(old: Vec<Literal>) -> OutputType {
        OutputType::Vec(old)
    }
}

impl From<String> for Basic {
    fn from(old: String) -> Basic {
        Basic {
            value: OutputType::from(old),
        }
    }
}

impl From<i64> for Basic {
    fn from(old: i64) -> Basic {
        Basic {
            value: OutputType::from(old),
        }
    }
}

impl From<f64> for Basic {
    fn from(old: f64) -> Basic {
        Basic {
            value: OutputType::from(old),
        }
    }
}

impl From<Vec<String>> for Basic {
    fn from(old: Vec<String>) -> Basic {
        let mut literals: Vec<Literal> = Vec::new();

        for i in old {
            literals.push(Literal::Str(i))
        }

        Basic {
            value: OutputType::from(literals),
        }
    }
}

impl From<Vec<i64>> for Basic {
    fn from(old: Vec<i64>) -> Basic {
        let mut literals: Vec<Literal> = Vec::new();

        for i in old {
            literals.push(Literal::Int(i))
        }

        Basic {
            value: OutputType::from(literals),
        }
    }
}

impl From<Vec<f64>> for Basic {
    fn from(old: Vec<f64>) -> Basic {
        let mut literals: Vec<Literal> = Vec::new();

        for i in old {
            literals.push(Literal::Float(i))
        }

        Basic {
            value: OutputType::from(literals),
        }
    }
}

struct Basic {
    value: OutputType,
}

fn check_type(var: Basic) {
    match var.value {
        OutputType::Literal(i) => match i {
            Literal::Str(e) => {
                println!("Str: {}", e)
            }
            Literal::Int(e) => {
                println!("Int: {}", e)
            }
            Literal::Float(e) => {
                println!("Float: {}", e)
            }
            Literal::Path(e) => {
                println!("Path: {:#?}", e.parts)
            }
        },
        OutputType::Vec(i) => {
            println!("Vec: {:#?}", i)
        }
    }
}

fn main() {
    let s = Basic::from("nice".to_string());
    let i = Basic::from(32);
    let f = Basic::from(13.7);
    let v = Basic::from(vec![1, 2, 3, 3, 4, 5]);

    println!("{}", s.value < i.value);
    println!("{}", s.value < "dice".to_owned());
    println!("{}", "rice".to_owned() > s.value);
    println!("{}", s.value == "nice".to_owned());
    check_type(s);
    check_type(i);
    check_type(f);
    check_type(v);
}
*/
