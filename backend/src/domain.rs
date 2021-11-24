#[cfg(feature = "domain_in_memory")]
mod in_memory;

use std::error::Error;

pub trait CrudRepository<Id, Item> {
    fn create(&mut self, item: &Item) -> Result<Id, Box<dyn Error>>;
    fn read(&self, id: &Id) -> Result<Option<Item>, Box<dyn Error>>;
    fn update(&mut self, id: &Id, item: &Item) -> Result<Option<Item>, Box<dyn Error>>;
    fn delete(&mut self, id: &Id) -> Result<Option<Item>, Box<dyn Error>>;
}
