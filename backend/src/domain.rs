#[cfg(feature = "domain_in_memory")]
mod in_memory;

use std::error::Error;

pub trait Repository<Id, Item> {
    fn create(&mut self, item: &Item) -> Result<Id, Box<dyn Error>>;
    fn get(&self, id: &Id) -> Result<Option<Item>, Box<dyn Error>>;
}
