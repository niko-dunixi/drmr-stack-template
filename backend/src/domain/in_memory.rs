use super::Repository;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use uuid::Uuid;

pub struct InMemory<T: 'static + Clone> {
    data: HashMap<String, T>,
}

impl<Item: 'static + Clone> InMemory<Item> {
    pub fn new() -> Box<dyn Repository<String, Item>> {
        Box::new(InMemory::<Item> {
            data: HashMap::new(),
        })
    }
}

impl<Item: 'static + Clone> Repository<String, Item>
    for InMemory<Item>
{
    fn create(&mut self, item: &Item) -> Result<String, Box<dyn Error>> {
        let id = Uuid::new_v4().to_hyphenated().to_string();
        self.data.insert(id.clone(), item.clone());
        Ok(id)
    }

    fn get(&self, id: &String) -> Result<Option<Item>, Box<dyn Error>> {
        Ok(self.data.get(id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_ok,assert_ok_eq,assert_some,assert_some_eq};

    #[test]
    fn new_in_memory_repo() {
        let _repo: Box<dyn Repository<String, String>> = InMemory::new();
    }

    #[test]
    fn get_is_ok_none() {
        let repo: Box<dyn Repository<String, String>> = InMemory::new();
        let id = String::from("my-id-foo");
        assert_ok_eq!(repo.get(&id), None);
    }

    #[test]
    fn get_is_ok_some() {
        let mut repo: Box<dyn Repository<String, String>> = InMemory::new();
        let item = String::from("Paul FREAKN Baker");
        let create_result = repo.create(&item);
        assert_ok!(&create_result);
        let id = create_result.unwrap();
        assert_ok_eq!(repo.get(&id), Some(item));
    }
}
