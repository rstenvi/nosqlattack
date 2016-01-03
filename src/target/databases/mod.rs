



/// Trait that individual DB authentication attacks must implement

use error::DBError;

pub trait DBAttack  {
    fn get_name(&self) -> &str;
    fn list_databases(&mut self) -> Result<Vec<String>,DBError>;
}



pub mod mongodb;
pub mod couchdb;

