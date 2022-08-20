use almond::{BaseOutput, BaseOutputGroup, Output};

fn get() -> impl Output<Group = BaseOutputGroup> {
    return BaseOutput::Str("epic and cool".to_owned());
}

fn main() {
    let s = BaseOutput::Str("nice".to_owned());
    let b = BaseOutput::Bool(true);
    let i = BaseOutput::Int(123456);
    let f = BaseOutput::Float(123.456);
    let n = get();

    println!("{}: {}", s, s.len());
    println!("{}: {}", b, b.len());
    println!("{}: {}", i, i.len());
    println!("{}: {}", f, f.len());
    println!("{}: {}", n, n.len());
}
