use core::cmp::Ordering;

pub enum OutputType {
    Literal,
    Vec(Vec<Literal>),
}

pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Ref(),
}

pub trait Variable {
    fn get(&self) -> OutputType;
}

pub struct Basic {
    pub value: OutputType,
}

impl Variable for Basic {}

impl Ord for Basic {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = self.get();
        let other_val = other.get();
    }
}

impl PartialOrd for Basic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {}
}

impl Eq for Basic {}

impl PartialEq for Basic {
    fn eq(&self, other: &Self) {}
}

/*
// =============== Output = String ===============

impl Ord for dyn Variable<Output = String> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(other.get())
    }
}

impl PartialOrd for dyn Variable<Output = String> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Variable<Output = i64>> PartialOrd<T> for dyn Variable<Output = String>
where
    (dyn Variable<Output = String>): PartialEq<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.get().cmp(&other.get().to_string()))
    }
}

impl Eq for dyn Variable<Output = String> {}

impl PartialEq for dyn Variable<Output = String> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: Variable<Output = i64>> PartialEq<T> for dyn Variable<Output = String>
where
    (dyn Variable<Output = String> + 'static): PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == &other.get().to_string()
    }
}

// =============== Output = i64 ===============

impl Ord for dyn Variable<Output = i64> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(other.get())
    }
}

impl PartialOrd for dyn Variable<Output = i64> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Variable<Output = String>> PartialOrd<T> for dyn Variable<Output = i64>
where
    (dyn Variable<Output = i64>): PartialEq<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.get().to_string().cmp(other.get()))
    }
}

impl Eq for dyn Variable<Output = i64> {}

impl PartialEq for dyn Variable<Output = i64> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: Variable<Output = String>> PartialEq<T> for dyn Variable<Output = i64>
where
    (dyn Variable<Output = i64> + 'static): PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        &self.get().to_string() == other.get()
    }
}

// =============== BasicString ===============

impl Variable for BasicString {
    type Output = String;

    fn get(&self) -> &String {
        &self.value
    }
}
*/
