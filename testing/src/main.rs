pub enum OutputType {
    Literal(Literal),
    Vec(Vec<Literal>),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
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

impl From<bool> for OutputType {
    fn from(old: bool) -> OutputType {
        OutputType::Literal(Literal::Bool(old))
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

impl From<bool> for Basic {
    fn from(old: bool) -> Basic {
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

impl From<Vec<bool>> for Basic {
    fn from(old: Vec<bool>) -> Basic {
        let mut literals: Vec<Literal> = Vec::new();

        for i in old {
            literals.push(Literal::Bool(i))
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
            Literal::Bool(e) => {
                println!("Bool: {}", e)
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
    let b = Basic::from(true);
    let v = Basic::from(vec![1, 2, 3, 3, 4, 5]);

    check_type(s);
    check_type(i);
    check_type(f);
    check_type(b);
    check_type(v);
}
