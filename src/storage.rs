pub trait GenericStorage<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn push(&mut self, value: T) -> usize;
    fn get(&self, index: usize) -> &T;
    fn len(&self) -> usize;
    fn remove(&mut self, index: usize) -> T;
}

impl<T> GenericStorage<T> for Vec<T> {
    fn new() -> Self {
        return Vec::new();
    }

    fn push(&mut self, value: T) -> usize {
        Vec::push(self, value);
        self.len() - 1
    }

    fn get(&self, index: usize) -> &T {
        &self[index]
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
    
}