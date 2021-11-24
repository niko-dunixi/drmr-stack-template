use super::Repository;

pub struct InMemory<T> {
    data: Vec<T>,
}

impl <T: 'static> InMemory<T> {
    pub fn new() -> Box<dyn Repository<T>> {
        Box::new(InMemory::<T>{
            data: vec!(),
        })
    }
}

impl <T> Repository<T> for InMemory<T> {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_in_memory_repo() {
        let _repo: Box<dyn Repository<String>> = InMemory::new();
    }
}
