// use core::cmp::Ordering;
use std::fmt::{self, Display};

pub type BaseOutputGroup = bool;
pub type VecOutputGroup = bool;

#[derive(Clone, Debug)]
pub enum BaseOutput {
    Str(String),
    Bool(bool),
    Int(i32),
    Float(f64),
    Path(RefPath),
}

#[derive(Clone, Debug)]
pub struct VecOutput<T: Output> {
    pub items: Vec<T>,
}

pub trait Output: Display {
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
    type Group = BaseOutputGroup;

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

impl Display for BaseOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            BaseOutput::Str(e) => return write!(f, "{}", e),
            BaseOutput::Bool(e) => e.to_string(),
            BaseOutput::Int(e) => e.to_string(),
            BaseOutput::Float(e) => e.to_string(),
            BaseOutput::Path(e) => e.to_string(),
        };

        write!(f, "{}", out)
    }
}

impl<T: Output> Output for VecOutput<T> {
    type Group = VecOutputGroup;

    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Output> Display for VecOutput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut counter = 0;
        let mut out: String = "".to_owned();

        for i in self.items.iter() {
            out.push('/');
            out.push_str(&i.to_string());
            counter += 1;

            if counter == 10 {
                break;
            }
        }

        write!(f, "{}", out)
    }
}

impl Display for RefPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = "".to_owned();

        for i in self.parts.iter() {
            out.push('/');
            out.push_str(i.as_str())
        }

        write!(f, "{}", out)
    }
}
