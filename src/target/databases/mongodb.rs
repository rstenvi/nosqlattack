
use std::result::Result::{Err,Ok};
use error::DBError;
use mongodb::{Client, ThreadedClient};
use std::option::Option;

pub struct MongoDB  {
    ip: String,
    port: u16,
    client: Option<Client>,
}

impl MongoDB    {
    pub fn new(ip: &str, port: u16) -> MongoDB {
        MongoDB {
            ip: ip.to_string(),
            port: port,
            client: None,
        }
    }
}


impl super::DBAttack for MongoDB   {
    fn get_name(&self) -> &str  {
        "MongoDB"
    }
    fn list_databases(&mut self) -> Result<Vec<String>,DBError> {
        self.client = match Client::connect(&self.ip, self.port) {
            Ok(o)     => Some(o),
            Err(e)    => {
                warn!("Unable to connect {}", e);
                return Err(DBError::Connection);
            }
        };
        match self.client {
            Some(ref b) => {
                match b.database_names()   {
                    Ok(bb) => Ok(bb),
                    Err(e) => {
                        warn!("Unable to list DB names {}", e);
                        return Err(DBError::Connection);
                    }
                }
            }
            None => {
                return Err(DBError::Connection)
            }
        }
    }
}

