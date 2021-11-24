use super::CrudRepository;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use uuid::Uuid;

pub struct InMemory<T: 'static + Clone> {
    data: HashMap<String, T>,
}

impl<Item: 'static + Clone> InMemory<Item> {
    pub fn new() -> Box<dyn CrudRepository<String, Item>> {
        Box::new(InMemory::<Item> {
            data: HashMap::new(),
        })
    }
}

impl<Item: 'static + Clone> CrudRepository<String, Item>
    for InMemory<Item>
{
    fn create(&mut self, item: &Item) -> Result<String, Box<dyn Error>> {
        let id = Uuid::new_v4().to_hyphenated().to_string();
        self.data.insert(id.clone(), item.clone());
        Ok(id)
    }

    fn read(&self, id: &String) -> Result<Option<Item>, Box<dyn Error>> {
        Ok(self.data.get(id).cloned())
    }

    fn update(&mut self, id: &String, item: &Item) -> Result<Option<Item>, Box<dyn Error>> {
        Ok(self.data.insert(id.clone(), item.clone()))
    }

    fn delete(&mut self, id: &String) -> Result<Option<Item>, Box<dyn Error>> {
        Ok(self.data.remove(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_ok,assert_ok_eq,assert_some,assert_some_eq};

    #[test]
    fn new_in_memory_repo() {
        let _repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
    }

    #[test]
    fn read_is_none() {
        let repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
        let id = String::from("my-id-foo");
        assert_ok_eq!(repo.read(&id), None);
    }

    #[test]
    fn read_is_some() {
        let mut repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
        let item = String::from("foo");
        let create_result = repo.create(&item);
        assert_ok!(&create_result);

        let id = create_result.unwrap();
        assert_ok_eq!(repo.read(&id), Some(item));
    }

    #[test]
    fn create_read_update() {
        let mut repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
        let item_foo = String::from("foo");
        let create_result = repo.create(&item_foo);
        assert_ok!(&create_result);

        let id = create_result.unwrap();
        assert_ok_eq!(repo.read(&id), Some(item_foo.clone()));

        let item_bar = String::from("bar");
        assert_ok_eq!(repo.update(&id, &item_bar), Some(item_foo));
        assert_ok_eq!(repo.read(&id), Some(item_bar));
    }

    #[test]
    fn delete_none() {
        let mut repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
        let id = String::from("my-id-foo");
        let delete_result = repo.delete(&id);
        assert_ok_eq!(delete_result, None);
    }

    #[test]
    fn delete_some() {
        let mut repo: Box<dyn CrudRepository<String, String>> = InMemory::new();
        let item_foo = String::from("foo");
        let create_result = repo.create(&item_foo);
        assert_ok!(&create_result);

        let id = create_result.unwrap();
        assert_ok_eq!(repo.delete(&id), Some(item_foo));
        assert_ok_eq!(repo.delete(&id), None);
    }
}
