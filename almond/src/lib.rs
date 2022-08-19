use core::cmp::Ordering;

pub trait Variable {
    type Output;

    fn get(&self) -> &Self::Output;
}

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

pub struct BasicString {
    pub value: String,
}

impl Variable for BasicString {
    type Output = String;

    fn get(&self) -> &String {
        &self.value
    }
}
