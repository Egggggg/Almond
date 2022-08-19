use crate;

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

impl<T: Variable<Output = u64>> PartialOrd<T> for dyn Variable<Output = String>
where
    (dyn Variable<Output = String> + 'static): PartialEq<T>,
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

impl<T: Variable<Output = u64>> PartialEq<T> for dyn Variable<Output = String>
where
    (dyn Variable<Output = String> + 'static): PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == &other.get().to_string()
    }
}
