use almond;

struct BasicInt {
    value: i64,
}

impl almond::Variable for BasicInt {
    type Output = i64;

    fn get(&self) -> &i64 {
        &self.value
    }
}

fn main() {
    let nice = almond::BasicString {
        value: "nice".to_string(),
    };
    let dice = almond::BasicString {
        value: "dice".to_string(),
    };
    let numer_str = almond::BasicString {
        value: "36".to_string(),
    };

    let numer = BasicInt { value: 37 };

    println!("{}", nice < dice);
    println!("{:#?}", nice < numer);
}
