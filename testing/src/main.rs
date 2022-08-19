use almond::{BaseOutput, Output};

fn get() -> Output {
    return BaseOutput::Str("nice");
}

fn main() {
    let s = BaseOutput::Str("nice".to_owned());
    let b = BaseOutput::Bool(true);
    let i = BaseOutput::Int(123456);
    let f = BaseOutput::Float(123.456);

    println!("{}", s.len());
    println!("{}", b.len());
    println!("{}", i.len());
    println!("{}", f.len());
}
