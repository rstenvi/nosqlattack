use std::result::Result::{Err,Ok};
use error::DBError;
use hyper;
use hyper::header::Connection;
use std::io::Read;
use rustc_serialize::json;

pub struct CouchDB  {
    client: hyper::Client,
    ip: String,
    port: u16,
}

impl CouchDB    {
    pub fn new(ip: &str, port: u16) -> CouchDB {
        CouchDB {
            client: hyper::Client::new(),
            ip: ip.to_string(),
            port: port,
        }
    }
    fn send_get(&self, url: &str) -> Result<String,DBError>    {
        debug!("Sending request: {}", url);
        let mut res = match self.client.get(url).send()   {
            Ok(o) => o,
            Err(e) => {
                warn!("CouchDB Error: {}", e);
                return Err(DBError::Connection);
            }
        };

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        Ok(body)
    }
}


// The trait implementation
impl super::DBAttack for CouchDB   {
    fn get_name(&self) -> &str  {
        "CouchDB"
    }
    fn list_databases(&mut self) -> Result<Vec<String>,DBError> {
        let url = format!("http://{}:{}/_all_dbs", self.ip, self.port);
        
        let data = match self.send_get(&url) {
            Ok(o) => o,
            Err(e) => {
                warn!("Unable to connect to DB: {:?}", e);
                return Err(DBError::Connection);
            }
        };

        let decoded: Vec<String> = json::decode(&*data).unwrap();
        debug!("Decoded response is {:?}", decoded);

        Ok(decoded)
    }
}
